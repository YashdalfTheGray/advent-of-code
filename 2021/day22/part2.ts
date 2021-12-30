// This file is generated by the create-files script in ../utils.
// Day 22 part 2
// https://adventofcode.com/2021/day/22#part2
// input: day22/input.txt

import Cuboid from './Cuboid.ts';
import Point from './Point.ts';

const d22p2Input = (await Deno.readTextFile('day22/input.txt'))
  .split('\n')
  .filter((l) => !!l);

const gridMap = new Map<string, boolean>();
const cuboids = d22p2Input.map((l) => new Cuboid(l));

cuboids.forEach((c) => {
  [...c]
    .map(([x, y, z]) => Point.encode(x, y, z))
    .forEach((p) => {
      gridMap.set(p, c.state);
    });
});

const cubesInOnState = [...gridMap.entries()].filter(
  ([, state]) => state
).length;

console.log('\n');
console.log(
  `After initialization, ${cubesInOnState} cubes are in the on state.`
);