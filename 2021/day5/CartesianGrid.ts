export default class CartesianGrid {
  private grid: Map<string, number> = new Map();
  private gridWidth = 0;
  private gridLength = 0;
  private maximumValue = 0;
  private stats = {
    horizontalLines: 0,
    verticalLines: 0,
    diagonalLines: {
      pxpy: 0,
      nxpy: 0,
      pxny: 0,
      nxny: 0,
    },
  };

  constructor() {}

  public plotLine(
    x1: number,
    y1: number,
    x2: number,
    y2: number,
    ignoreDiagonals = true
  ) {
    if (x1 === x2 && y1 === y2) {
      throw new TypeError(
        'Cannot plot a line with the same start and end point'
      );
    }

    if (x1 === x2) {
      this.plotVerticalLine(x1, y1, y2);
      this.stats.verticalLines += 1;
    } else if (y1 === y2) {
      this.plotHorizontalLine(x1, x2, y2);
      this.stats.horizontalLines += 1;
    } else {
      if (!ignoreDiagonals) {
        this.plotDiagonalLine(x1, y1, x2, y2);
      }
    }

    this.gridWidth = Math.max(this.gridWidth, x1, x2);
    this.gridLength = Math.max(this.gridLength, y1, y2);
  }

  public get dimensions() {
    return [this.gridWidth, this.gridLength];
  }

  public get maximunValueSeen() {
    return this.maximumValue;
  }

  public get statistics() {
    return this.stats;
  }

  public entries() {
    return this.grid.entries();
  }

  private plotVerticalLine(x: number, y1: number, y2: number) {
    for (let i = Math.min(y1, y2); i <= Math.max(y1, y2); i++) {
      this.plotPoint(x, i);
    }
  }

  private plotHorizontalLine(x1: number, x2: number, y: number) {
    for (let i = Math.min(x1, x2); i <= Math.max(x1, x2); i++) {
      this.plotPoint(i, y);
    }
  }

  private plotDiagonalLine(x1: number, y1: number, x2: number, y2: number) {
    const xSlope = Math.max(x1, x2) - Math.min(x1, x2);
    const ySlope = Math.max(y1, y2) - Math.min(y1, y2);
    // 2021-12-05 @YashdalfTheGray:
    // This makes the assumption that we are only ever going
    // to encounter a diagonal with |xSlope| === |ySlope|
    const unitSlope = xSlope / ySlope;
    const xDirection = x1 < x2 ? 1 : -1;
    const yDirection = y1 < y2 ? 1 : -1;

    const keyName = `${xDirection === 1 ? 'p' : 'n'}x${
      xDirection === 1 ? 'p' : 'n'
    }y`;

    this.statistics.diagonalLines[
      keyName as 'pxpy' | 'nxpy' | 'pxny' | 'nxny'
    ] += 1;

    let [x, y] = [x1, y1];
    while (x !== x2 || y !== y2) {
      this.plotPoint(x, y);
      x += unitSlope * xDirection;
      y += unitSlope * yDirection;
    }
  }

  private plotPoint(x: number, y: number) {
    const key = `(${x},${y})`;
    const currentValue = this.grid.get(key) || 0;
    this.grid.set(key, currentValue + 1);
    this.maximumValue = Math.max(this.maximumValue, currentValue + 1);
  }
}
