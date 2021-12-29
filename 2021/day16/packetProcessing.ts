export class RawNumericBuffer {
  private readHead = 0;

  constructor(private data: string) {}

  public get size(): number {
    return this.data.length;
  }

  public get unreadSize(): number {
    return this.size - this.readHead;
  }

  public isAllZeros(): boolean {
    return this.data
      .substr(this.readHead)
      .split('')
      .every((c) => c === '0');
  }

  public isEmpty(): boolean {
    return this.readHead === this.size - 1;
  }

  public read(length: number): string;
  public read(length: number, radix: number): number;
  public read(length: number, radix?: number): string | number {
    if (this.data.length === 0) {
      throw new Error('Cannot read from empty buffer');
    }

    const readStr =
      this.size < length || this.unreadSize < length
        ? this.data.substr(this.readHead)
        : this.data.substr(this.readHead, length);
    this.readHead += length;

    return radix ? parseInt(readStr, radix) : readStr;
  }

  public flush(): string {
    const flushed = this.data.substr(this.readHead);
    this.readHead = this.size - 1;
    return flushed;
  }
}

export class PacketData {
  constructor(
    public version: number,
    public type: number,
    public result = 0,
    public subPackets: PacketData[] = []
  ) {}

  public versionSum(): number {
    return this.subPackets.reduce(
      (sum, p) => sum + p.versionSum(),
      this.version
    );
  }
}

export const parsePackets = (buffer: RawNumericBuffer): PacketData => {
  const rootPacket = new PacketData(buffer.read(3, 2), buffer.read(3, 2));

  if (rootPacket.type === 4) {
    let result = '';

    while (true) {
      const currentGroup = buffer.read(5);
      result += currentGroup.substr(1);

      if (currentGroup.substr(0, 1) === '0') {
        break;
      }
    }

    rootPacket.result = parseInt(result, 2);
  } else {
    const lengthType = buffer.read(1, 2);

    if (lengthType === 0) {
      let totalSubPacketLength = buffer.read(15, 2);
      let startingPosition = buffer.unreadSize;

      while (totalSubPacketLength > 0) {
        rootPacket.subPackets.push(parsePackets(buffer));
        totalSubPacketLength -= startingPosition - buffer.unreadSize;
        startingPosition = buffer.unreadSize;
      }
    } else if (lengthType === 1) {
      const totalPackets = buffer.read(11, 2);

      for (let i = 0; i < totalPackets; i++) {
        rootPacket.subPackets.push(parsePackets(buffer));
      }
    }
  }

  return rootPacket;
};
