// Day 3 part 2
// https://adventofcode.com/2021/day/3#part2
// input: day3/input.txt

const getTotalsForColumn = (
  lines: number[][],
  column: number
): [number, number] => {
  const totals: [number, number] = [0, 0];

  lines.forEach((line) => {
    totals[line[column]] += 1;
  });

  return totals;
};

const getOxygenGeneratorRating = (lines: number[][]): number => {
  let currentColumn = 0;
  let filteredLines = lines;

  while (filteredLines.length > 1) {
    const [zeroTotal, oneTotal] = getTotalsForColumn(
      filteredLines,
      currentColumn
    );

    if (oneTotal >= zeroTotal) {
      filteredLines = filteredLines.filter((line) => line[currentColumn] === 1);
    } else {
      filteredLines = filteredLines.filter((line) => line[currentColumn] === 0);
    }

    currentColumn += 1;
  }

  return parseInt(filteredLines[0].join(''), 2);
};

const getCo2ScrubberRating = (lines: number[][]): number => {
  let currentColumn = 0;
  let filteredLines = lines;

  while (filteredLines.length > 1) {
    const [zeroTotal, oneTotal] = getTotalsForColumn(
      filteredLines,
      currentColumn
    );

    if (oneTotal < zeroTotal) {
      filteredLines = filteredLines.filter((line) => line[currentColumn] === 1);
    } else {
      filteredLines = filteredLines.filter((line) => line[currentColumn] === 0);
    }

    currentColumn += 1;
  }

  return parseInt(filteredLines[0].join(''), 2);
};

const reportLines = (await Deno.readTextFile('day3/input.txt'))
  .split('\n')
  .map((line) => line.split('').map((c) => parseInt(c, 10)));

const oxygenGeneratorRating = getOxygenGeneratorRating(reportLines);
const co2ScrubberRating = getCo2ScrubberRating(reportLines);

console.log('\n');
console.log(`The oxygen generator rating is ${oxygenGeneratorRating}.`);
console.log(`The Co2 scrubber rating is ${co2ScrubberRating}.`);
console.log(
  `The life support rating is ${oxygenGeneratorRating * co2ScrubberRating}.`
);
