// This file is generated by the create-files script in ../utils.
// Day 8 part 1
// https://adventofcode.com/2021/day/8
// input: day8/input.txt

class SevenSegmentExperimentPartial {
  private rawReadings: string[] = [];
  private rawOutput: string[] = [];
  private parsedOutput: number[] = [];

  constructor(entry: string) {
    const [readingsStr, outputStr] = entry.split('|').map((s) => s.trim());
    this.rawReadings = readingsStr.split(' ').map((s) => s.trim());
    this.rawOutput = outputStr.split(' ').map((s) => s.trim());
  }

  public get rawReadingStrings(): string[] {
    return this.rawReadings;
  }

  public get rawOutputStrings(): string[] {
    return this.rawOutput;
  }
}

const d8p1Input = (await Deno.readTextFile('day8/input.txt'))
  .split('\n')
  .filter((l) => !!l)
  .map((l) => new SevenSegmentExperimentPartial(l));

const easyNumberCount = d8p1Input.reduce((count, experiment) => {
  const rawOutput = experiment.rawOutputStrings;
  return (
    count + rawOutput.filter((s) => [2, 3, 4, 7].includes(s.length)).length
  );
}, 0);

console.log(`1, 4, 7, and 8 appear ${easyNumberCount} times in the output.`);
