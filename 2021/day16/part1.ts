// This file is generated by the create-files script in ../utils.
// Day 16 part 1
// https://adventofcode.com/2021/day/16
// input: day16/input.txt

const testInput1 = `D2FE28`;
const testInput2 = `38006F45291200`;
const testInput3 = `EE00D40C823060`;

class Packet {
  #version: number;
  #type: number;
  #result = 0;
  #subPackets = new Array<Packet>();

  constructor(input: string) {
    // the first three bits are the version
    this.#version = parseInt(input.substr(0, 3), 2);
    // the next three bits are the type
    this.#type = parseInt(input.substr(3, 3), 2);

    switch (this.#type) {
      case 4:
        this.parseLiteral(input.substr(6));
        break;
      default:
        this.parseOperation(input.substr(6));
    }
  }

  public get version() {
    return this.#version;
  }

  public get type() {
    return this.#type;
  }

  public get result() {
    return this.#result;
  }

  public toString(): string {
    return `ver ${this.#version} type ${this.#type}, result ${this.#result}${
      this.#subPackets.length !== 0
        ? '\nSubpackets:\n' +
          this.#subPackets.map((p) => `  - ${p.toString()}`).join('\n')
        : ''
    }`;
  }

  private parseLiteral(input: string) {
    const groups = this.splitStringIntoGroups(input);

    let result = '';

    for (const g of groups) {
      if (g.length === 5) {
        result += g.substr(1);
      }

      if (g.substr(0, 1) === '0') {
        break;
      }
    }

    this.#result = parseInt(result, 2);
  }

  private parseOperation(input: string) {
    const lengthType = parseInt(input.substr(0, 1), 2);
    if (lengthType === 0) {
      const totalSubPacketLength = parseInt(input.substr(1, 15), 2);
      console.log(totalSubPacketLength, input.substr(16));
    } else if (lengthType === 1) {
      const totalPackets = parseInt(input.substr(1, 11), 2);
      console.log(totalPackets, input.substr(12));
    }
  }

  private splitStringIntoGroups(input: string, groupSize = 5): string[] {
    return input.match(new RegExp(`.{1,${groupSize}}`, 'g'))!;
  }
}

const d16p1Input = [
  testInput1,
  testInput2,
  testInput3,
  // await Deno.readTextFile('day16/input.txt'),
]
  .map((i) =>
    i
      .split('')
      .map((h) => parseInt(h, 16).toString(2).padStart(4, '0'))
      .join('')
  )
  .map((i) => new Packet(i));

d16p1Input.forEach((p) => console.log(p.toString()));
