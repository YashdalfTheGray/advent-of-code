// Day 1 part 2
// https://adventofcode.com/2021/day/1#part2
// input: day1/input.txt

let slidingWindowDepthIncreases = 0;

const measures = (await Deno.readTextFile('day1/input.txt'))
  .split('\n')
  .map((m) => parseInt(m, 10));

for (let i = 0; i < measures.length; i++) {
  const windowA = measures.slice(i, i + 3);
  const windowB = measures.slice(i + 1, i + 4);

  const windowASum = windowA.reduce((a, b) => a + b, 0);
  const windowBSum = windowB.reduce((a, b) => a + b, 0);

  if (windowBSum > windowASum) {
    slidingWindowDepthIncreases++;
  }
}

console.log(
  `\nThe depth increased ${slidingWindowDepthIncreases} times using a 3-measurement sliding window!`
);
