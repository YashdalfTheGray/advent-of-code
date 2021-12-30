// This file is generated by the create-files script in ../utils.
// Day 16 part 2
// https://adventofcode.com/2021/day/16#part2
// input: day16/input.txt

import { RawNumericBuffer, parsePackets } from './packetProcessing.ts';

// A fun way to visualize and create more test cases here
// https://davidyue.live/aoc/app/packetcode.html

const d16p2Input = [
  'D2FE28',
  'C200B40A82',
  '04005AC33890',
  '880086C3E88112',
  'CE00C43D881120',
  'D8005AC2A8F0',
  'F600BC2D8F',
  '9C005AC2F8F0',
  '9C0141080250320F1802104A08',
  await Deno.readTextFile('day16/input.txt'),
].map((i) =>
  i
    .split('')
    .map((h) => parseInt(h, 16).toString(2).padStart(4, '0'))
    .join('')
);

const parsedPackets = d16p2Input
  .map((i) => new RawNumericBuffer(i))
  .map((b) => parsePackets(b));

console.log(parsedPackets.map((p) => p.resolve()));