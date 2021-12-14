export type Point = { x: number; y: number };
export type Fold = { foldAxis: 'x' | 'y'; foldPosition: number };

export class PointGrid {
  private grid: Set<string>;
  private height = 0;
  private width = 0;

  constructor(input: { x: number; y: number }[]) {
    this.grid = new Set();
    for (const point of input) {
      if (point.x > this.width) {
        this.width = point.x;
      }
      if (point.y > this.height) {
        this.height = point.y;
      }

      this.grid.add(this.encodePoint(point.x, point.y));
    }
  }

  public get knownPointsSize(): number {
    return this.grid.size;
  }

  public display(): string {
    let output = '';
    for (let y = 0; y <= this.height; y++) {
      for (let x = 0; x <= this.width; x++) {
        if (this.grid.has(this.encodePoint(x, y))) {
          output += '#';
        } else {
          output += '.';
        }
      }
      output += '\n';
    }
    return output;
  }

  public applyFold(fold: Fold) {
    if (fold.foldAxis === 'x') {
      this.applyFoldX(fold.foldPosition);
    } else {
      this.applyFoldY(fold.foldPosition);
    }
  }

  private applyFoldX(foldPosition: number) {
    this.grid.forEach((point) => {
      const { x, y } = this.decodePoint(point);

      if (x > foldPosition) {
        const newX = foldPosition - (x - foldPosition);
        this.grid.add(this.encodePoint(newX, y));
        this.grid.delete(point);
      }
    });

    this.width = foldPosition;
  }

  private applyFoldY(foldPosition: number) {
    this.grid.forEach((point) => {
      const { x, y } = this.decodePoint(point);

      if (y > foldPosition) {
        const newY = foldPosition - (y - foldPosition);
        this.grid.add(this.encodePoint(x, newY));
        this.grid.delete(point);
      }
    });

    this.height = foldPosition;
  }

  private encodePoint(x: number, y: number): string {
    return `${x},${y}`;
  }

  private decodePoint(point: string): { x: number; y: number } {
    const [x, y] = point.split(',');
    return { x: parseInt(x, 10), y: parseInt(y, 10) };
  }
}
