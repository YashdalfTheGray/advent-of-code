// This file is generated by the setup script located in the utils folder in the repository root.
// Run the tests using deno test --allow-read=input.txt solution.test.ts

import {
  assertExists,
  assertEquals,
} from 'https://deno.land/std/testing/asserts.ts';

import { day6Input, parseCommand, part1, part2 } from './solution.ts';

Deno.test('sanity', () => {
  assertExists(day6Input);
});

[
  {
    input: 'turn on 0,0 through 999,999',
    output: {
      action: 'turn on',
      start: [0, 0],
      end: [999, 999],
    },
  },
  {
    input: 'toggle 0,0 through 999,0',
    output: {
      action: 'toggle',
      start: [0, 0],
      end: [999, 0],
    },
  },
].forEach((tc) => {
  Deno.test(`[parseCommand] parses ${tc.input} correctly`, () => {
    assertEquals(parseCommand(tc.input), tc.output);
  });
});

[
  { id: 1, input: ['turn on 0,0 through 0,0'], output: 1 },
  { id: 2, input: ['turn on 0,0 through 999,999'], output: 1000000 },
  { id: 3, input: ['toggle 0,0 through 999,0'], output: 1000 },
  {
    id: 4,
    input: [
      'turn on 0,0 through 999,0',
      'toggle 900,0 through 999,0',
      'turn off 0,0 through 9,0',
    ],
    output: 890,
  },
  { id: 5, input: day6Input, output: 569999 },
].forEach((tc) => {
  Deno.test(`[part1] returns ${tc.output} for input id ${tc.id}`, () => {
    assertEquals(part1(tc.input), tc.output);
  });
});

[
  { id: 1, input: ['turn on 0,0 through 0,0'], output: 1 },
  { id: 2, input: ['turn on 0,0 through 999,999'], output: 1000000 },
  { id: 3, input: ['toggle 0,0 through 999,999'], output: 2000000 },
  { id: 4, input: ['toggle 0,0 through 999,0'], output: 2000 },
  {
    id: 5,
    input: [
      'turn on 0,0 through 999,0',
      'toggle 900,0 through 999,0',
      'turn off 0,0 through 9,0',
    ],
    output: 1190,
  },
  { id: 6, input: day6Input, output: 17836115 },
].forEach((tc) => {
  Deno.test(`[part2] returns ${tc.output} for input id ${tc.id}`, () => {
    assertEquals(part2(tc.input), tc.output);
  });
});
