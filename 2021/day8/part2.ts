// This file is generated by the create-files script in ../utils.
// Day 8 part 2
// https://adventofcode.com/2021/day/8#part2
// input: day8/input.txt

enum Segment {
  Top,
  TopLeft,
  TopRight,
  Middle,
  BottomLeft,
  BottomRight,
  Bottom,
}

type SegmentDisplayState = {
  [key in 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 0]: string;
};
type ReverseSegmentDisplayState = {
  [key: string]: keyof SegmentDisplayState;
};
type SegmentToLetterMap = Partial<{
  [key in Segment]: string;
}>;

class SevenSegmentExperiment {
  private rawReadings: string[] = [];
  private rawOutput: string[] = [];
  private parsedOutput: number[] = [];
  private intermediateInternalState: SegmentDisplayState;
  private internalState: ReverseSegmentDisplayState;
  private workspaceContext: SegmentToLetterMap = {};

  constructor(entry: string) {
    const [readingsStr, outputStr] = entry.split('|').map((s) => s.trim());
    this.rawReadings = readingsStr.split(' ').map((s) => s.trim());
    this.rawOutput = outputStr.split(' ').map((s) => s.trim());

    this.intermediateInternalState = new Array(10)
      .fill(0)
      .reduce((state, _, i) => ({ ...state, [i]: '' }), {});

    this.internalState = {};
  }

  public get rawReadingStrings(): string[] {
    return this.rawReadings;
  }

  public get rawOutputStrings(): string[] {
    return this.rawOutput;
  }

  public parseOutput(): number[] {
    this.analyzeReadings();
    this.parsedOutput = this.rawOutput.map((s) => this.internalState[s]);
    return this.parsedOutput;
  }

  private analyzeReadings() {
    // find the easy ones like 1, 4, 7, 8
    this.firstParsingPass();
    // 3 is two segments away from 7
    this.secondParsingPass();
    // 9 is one segment away from 3
    this.thirdParsingPass();
    // next up are 2, 5, 6, and 0
    this.fourthParsingPass();

    // build the reverse lookup map
    this.internalState = Object.entries(this.intermediateInternalState).reduce(
      (state, [number, representation]) => ({
        ...state,
        [representation]: parseInt(number),
      }),
      {}
    );
  }

  private firstParsingPass() {
    this.rawReadings.forEach((reading) => {
      switch (reading.length) {
        case 2:
          this.intermediateInternalState[1] = reading;
          break;
        case 3:
          this.intermediateInternalState[7] = reading;
          break;
        case 4:
          this.intermediateInternalState[4] = reading;
          break;
        case 7:
          this.intermediateInternalState[8] = reading;
      }
    });
  }

  private secondParsingPass() {
    this.rawReadings
      .filter((r) => !Object.values(this.intermediateInternalState).includes(r))
      .forEach((reading) => {
        if (
          reading.length === 5 &&
          this.includesSubstringUnordered(
            reading,
            this.intermediateInternalState[7]
          )
        ) {
          this.intermediateInternalState[3] = reading;
        }
      });
  }

  private thirdParsingPass() {
    this.rawReadings
      .filter((r) => !Object.values(this.intermediateInternalState).includes(r))
      .forEach((reading) => {
        if (
          reading.length === 6 &&
          this.includesSubstringUnordered(
            reading,
            this.intermediateInternalState[3]
          )
        ) {
          this.intermediateInternalState[9] = reading;
          this.workspaceContext[Segment.TopLeft] = this.setDifference(
            this.intermediateInternalState[9],
            this.intermediateInternalState[3]
          )[0];
        }
      });
  }

  private fourthParsingPass() {
    this.rawReadings
      .filter((r) => !Object.values(this.intermediateInternalState).includes(r))
      .forEach((reading, i, arr) => {
        if (reading.length === 5) {
          const readingSetDifferenceFrom3 = this.setDifference(
            reading,
            this.intermediateInternalState[3]
          );

          // the segment set difference of 5 and 3 is the top left segment
          if (
            readingSetDifferenceFrom3.length === 1 &&
            readingSetDifferenceFrom3[0] ===
              this.workspaceContext[Segment.TopLeft]
          ) {
            this.intermediateInternalState[5] = reading;
          }
          // the segment set difference of 2 and 3 is the segment set
          // that is orthogonal to the segment set of 9
          if (
            readingSetDifferenceFrom3.length === 1 &&
            this.setDifference(
              this.intermediateInternalState[9],
              readingSetDifferenceFrom3.join('')
            )
          ) {
            this.intermediateInternalState[2] = reading;
          }
        } else if (reading.length === 6) {
          const readingSetDifferenceFrom8 = this.setDifference(
            this.intermediateInternalState[8],
            reading
          );

          // the segment set difference of 6 and 8 is the segment set
          // that is orthogonal to the segment set of 1 if the reading
          // is displaying 0
          if (
            this.setDifference(
              this.intermediateInternalState[1],
              readingSetDifferenceFrom8.join('')
            )
          ) {
            this.intermediateInternalState[0] = reading;
          } else {
            // if it is not orthogonal, that means it contains the
            // TopRight segment, which means the reading is a 6
            this.intermediateInternalState[6] = reading;
          }
        }
      });
  }

  private includesSubstringUnordered(str: string, sub: string): boolean {
    return sub.split('').filter((s) => !str.includes(s)).length === 0;
  }

  private setDifference(s1: string, s2: string): string[] {
    return s1.split('').filter((e) => !s2.includes(e));
  }
}

const d8p2Input = (await Deno.readTextFile('day8/input.txt'))
  .split('\n')
  .filter((l) => !!l)
  .map((l) => new SevenSegmentExperiment(l));

const stuff = d8p2Input[0].parseOutput();
console.log(stuff);
console.log(d8p2Input[0]);

// const totalOfAllTheOutput = d8p2Input.reduce(
//   (runningTotal, experiment) =>
//     runningTotal + experiment.parseOutput().reduce((sum, o) => sum + o, 0),
//   0
// );
// console.log(`The total of all parsed values is ${totalOfAllTheOutput}`);
