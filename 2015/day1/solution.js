// This file is generated by the setup script located in the utils folder in the repository root.
// Day 1
// https://adventofcode.com/2015/day/1
// input: day1/input.txt

const { readFileSync } = require('fs');

const day1Input = readFileSync('input.txt', 'utf8')
  .split('\n')
  .filter((l) => !!l)[0];

/**
 * Advent of code 2015 day1 part1
 * @param {string} input a string of ( and ) characters
 */
const part1 = (input) =>
  input
    .split('')
    .reduce(
      (currentFloor, char) =>
        char === '(' ? currentFloor + 1 : currentFloor - 1,
      0
    );

const part2 = (input) => {
  let floor = 0;

  for (let i = 0; i < input.length; i++) {
    const char = input[i];
    floor += char === '(' ? 1 : -1;
    if (floor < 0) {
      return i + 1;
    }
  }
};

module.exports = { day1Input, part1, part2 };