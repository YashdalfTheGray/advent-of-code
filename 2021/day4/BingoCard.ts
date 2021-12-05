type BingoCardInternalState = {
  [key: number]: { row: number; column: number; isChecked: boolean };
};

export default class BingoCard {
  private card: BingoCardInternalState;
  private rowTracker: number[];
  private columnTracker: number[];

  constructor(input: string[]) {
    this.rowTracker = Array(input.length).fill(0);
    this.columnTracker = Array(input.length).fill(0);
    this.card = input.reduce((board, currentRow, row) => {
      currentRow
        .split(' ')
        .filter((v) => !!v)
        .forEach((value, column) => {
          board[parseInt(value, 10)] = { row, column, isChecked: false };
        });

      return board;
    }, {} as BingoCardInternalState);
  }

  public check(calledNumber: number) {
    if (this.card[calledNumber]) {
      const { row, column } = this.card[calledNumber];

      this.rowTracker[row] += 1;
      this.columnTracker[column] += 1;
      this.card[calledNumber].isChecked = true;
    }
  }

  public get hasBingo(): boolean {
    return (
      this.rowTracker.some((v) => v === this.rowTracker.length) ||
      this.columnTracker.some((v) => v === this.columnTracker.length)
    );
  }

  public get sumOfUncheckedNumbers(): number {
    return Object.entries(this.card)
      .filter(([_, { isChecked }]) => !isChecked)
      .map(([k, _]) => parseInt(k, 10))
      .reduce((sum, num) => sum + num, 0);
  }

  public getCardScore(lastNumberCalled: number): number {
    return lastNumberCalled * this.sumOfUncheckedNumbers;
  }
}
