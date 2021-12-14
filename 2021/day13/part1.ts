// This file is generated by the create-files script in ../utils.
// Day 13 part 1
// https://adventofcode.com/2021/day/13
// input: day13/input.txt

import { PointGrid, Fold } from './PointGrid.ts';

const [d13p1Dots, d13p1Folds] = (await Deno.readTextFile('day13/input.txt'))
  .split('\n\n')
  .filter((l) => !!l);

const parsedDots = new PointGrid(
  d13p1Dots
    .split('\n')
    .filter((l) => !!l)
    .map((l) => {
      const [x, y] = l.split(',').map(Number);
      return { x, y };
    })
    .sort((a, b) => (a.x === b.x ? a.y - b.y : a.x - b.x))
);

const parsedFolds = d13p1Folds
  .split('\n')
  .filter((l) => !!l)
  .map((l) => {
    const [wholeMatch, foldAxis, foldPosition] =
      /^fold\salong\s(.)=(.*)$/m.exec(l)!;
    return { foldAxis, foldPosition: Number(foldPosition), raw: wholeMatch };
  });

parsedDots.applyFold(parsedFolds[0] as Fold);
console.log('\n');
console.log(
  `${parsedDots.knownPointsSize} points are visible after the first fold.`
);
