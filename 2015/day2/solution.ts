// This file is generated by the setup script located in the utils folder in the repository root.
// Day 2
// https://adventofcode.com/2015/day/2
// input: day2/input.txt

type GiftDimensions = {
  l: number;
  w: number;
  h: number;
};

const day2Input = (await Deno.readTextFile('input.txt'))
  .split('\n')
  .filter((l) => !!l)
  .map((l) => l.split('x'))
  .map(([ls, ws, hs]) => ({
    l: parseInt(ls, 10),
    w: parseInt(ws, 10),
    h: parseInt(hs, 10),
  }));

const perimeter = (l: number, h: number) => 2 * l + 2 * h;
const surfaceArea = (l: number, w: number, h: number) =>
  2 * l * w + 2 * w * h + 2 * h * l;

const part1 = (input: GiftDimensions[]): number =>
  input.reduce((total, { l, w, h }) => {
    const smallestSide = Math.min(l * w, w * h, h * l);
    return total + surfaceArea(l, w, h) + smallestSide;
  }, 0);

const part2 = (input: GiftDimensions[]): number =>
  input.reduce((totalRibbon, { l, w, h }) => {
    const smallestSide = Math.min(
      perimeter(l, w),
      perimeter(w, h),
      perimeter(h, l)
    );
    const bow = l * w * h;
    return totalRibbon + smallestSide + bow;
  }, 0);

export { day2Input, part1, part2 };