// This file is generated by the create-files script in ../utils.
// Day 20 part 2
// https://adventofcode.com/2021/day/20#part2
// input: day20/input.txt

import {
  addBorder,
  getSquareOfPixels,
  pixelsToNumber,
  determineNewDefaultValue,
} from './pixelUtils.ts';

const d20p2Input = (await Deno.readTextFile('day20/input.txt')).split('\n\n');

const imageProcessingMap = d20p2Input[0].trim().split('');
const image = d20p2Input[1]
  .split('\n')
  .filter(Boolean)
  .map((l) => l.split(''));

const algorithmPasses = 50;

let currentImage = image;
let defaultValue = '.';

for (let i = 0; i < algorithmPasses; i++) {
  currentImage = addBorder(currentImage, defaultValue);
  currentImage = currentImage.map((row, y) => {
    return row.map((_, x) => {
      const pixels = getSquareOfPixels(currentImage, x, y, defaultValue);
      const imageProcessingMapIndex = pixelsToNumber(pixels);
      return imageProcessingMap[imageProcessingMapIndex];
    });
  });
  defaultValue = determineNewDefaultValue(imageProcessingMap, defaultValue);
}

const litPixels = currentImage.flat().filter((p) => p === '#').length;

console.log('\n');
console.log(
  `There are ${litPixels} lit pixels in the image after ${algorithmPasses} passes.`
);
