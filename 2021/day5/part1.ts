// This file is generated by the create-files script in ../utils.
// Day 5 part 1
// https://adventofcode.com/2021/day/5
// input: day5/input.txt

import CartesianGrid from './CartesianGrid.ts';

const d5p1Input = (await Deno.readTextFile('day5/input.txt'))
  .split('\n')
  .filter((l) => !!l);
const grid = new CartesianGrid();

d5p1Input.forEach((line) => {
  const [p1, p2] = line.split('->').map((s) => s.trim());
  const [p1x, p1y] = p1.split(',').map((s) => parseInt(s, 10));
  const [p2x, p2y] = p2.split(',').map((s) => parseInt(s, 10));
  grid.plotLine(p1x, p1y, p2x, p2y);
});

const mostDangerousPoints = [...grid.entries()].filter(([_, v]) => v > 1);

console.log('\n');
console.log(
  `There are ${mostDangerousPoints.length} points that are not safe.`
);
