// This file is generated by the setup script located in the utils folder in the repository root.
// Run the tests using deno test --allow-read solution.test.ts

import {
  assertEquals,
  assertExists,
} from 'https://deno.land/std/testing/asserts.ts';

import { day2Input, part1, part2 } from './solution.ts';

Deno.test('sanity test', () => {
  assertExists(day2Input);
});

[
  { input: [{ l: 2, w: 3, h: 4 }], p1Output: 58, p2Output: 34 },
  { input: [{ l: 1, w: 1, h: 10 }], p1Output: 43, p2Output: 14 },
  { input: day2Input, p1Output: 1588178, p2Output: 3783758 },
].forEach((tc) => {
  Deno.test(
    `[part1] List of ${tc.input.length} presents require ${tc.p1Output} sq feet of wrapping paper`,
    () => {
      assertEquals(part1(tc.input), tc.p1Output);
    }
  );

  Deno.test(
    `[part2] List of ${tc.input.length} presents require ${tc.p2Output} feet of ribbon`,
    () => {
      assertEquals(part2(tc.input), tc.p2Output);
    }
  );
});
