// This file is generated by the setup script located in the utils folder in the repository root.
// Day 5
// https://adventofcode.com/2015/day/5
// input: day5/input.txt

const { readFileSync } = require('fs');

const day5Input = readFileSync('input.txt', 'utf8')
  .split('\n')
  .filter((l) => !!l);

/**
 * Advent of code 2015 day 5 part 1
 * @param {Array<string>} input the list of strings to evaluate
 */
const part1 = (input) =>
  input.reduce((niceCount, line) => {
    const hasThreeVowels = /[aeiou].*[aeiou].*[aeiou]/.test(line);
    const hasDoubleLetter = /(.)\1/.test(line);
    const hasBadSequence = /ab|cd|pq|xy/.test(line);
    return hasThreeVowels && hasDoubleLetter && !hasBadSequence
      ? niceCount + 1
      : niceCount;
  }, 0);

/**
 * Advent of code 2015 day 5 part 2
 * @param {Array<string>} input the list of strings to evaluate
 */
const part2 = (input) =>
  input.reduce((niceCount, line) => {
    const hasDoublePair = /(..).*\1/.test(line);
    const hasRepeatedLetterWithSpacing = /(.).\1/.test(line);
    return hasDoublePair && hasRepeatedLetterWithSpacing
      ? niceCount + 1
      : niceCount;
  }, 0);

module.exports = { day5Input, part1, part2 };
