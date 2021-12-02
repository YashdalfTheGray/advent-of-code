// Day 1 part 1
// https://adventofcode.com/2021/day/1
// input: day1/input.txt

const measurements = (await Deno.readTextFile('day1/input.txt')).split('\n');

const depthIncreasedCount = measurements
  .map((m) => parseInt(m, 10))
  .reduce((depthIncreasedCount, m, i, arr) => {
    if (i !== 0) {
      return m > arr[i - 1] ? depthIncreasedCount + 1 : depthIncreasedCount;
    } else {
      return 0;
    }
  }, 0);

console.log(`\nThe depth increased ${depthIncreasedCount} times!`);
