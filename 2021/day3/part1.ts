// Day 3 part 1
// https://adventofcode.com/2021/day/3
// input: day3/input.txt

const transpose = (matrix: number[][]) => {
  return Object.keys(matrix[0]).map((c) => {
    return matrix.map((r) => {
      return r[parseInt(c, 10)];
    });
  });
};

const calculateGammaRate = (totals: number[][]) =>
  parseInt(
    totals
      .reduce(
        (buffer, [zeroTotal, oneTotal]) =>
          buffer.concat(oneTotal > zeroTotal ? 1 : 0),
        []
      )
      .join(''),
    2
  );

const calculateEpsilonRate = (totals: number[][]) =>
  parseInt(
    totals
      .reduce(
        (buffer, [zeroTotal, oneTotal]) =>
          buffer.concat(oneTotal < zeroTotal ? 1 : 0),
        []
      )
      .join(''),
    2
  );

const reportMatrix = transpose(
  (await Deno.readTextFile('day3/input.txt'))
    .split('\n')
    .map((line) => line.split('').map((c) => parseInt(c, 10)))
);

const totals = reportMatrix.reduce((data, column, index) => {
  if (!data[index]) {
    data[index] = [0, 0];
  }

  column.reduce((dataRow, d) => {
    dataRow[d]++;
    return dataRow;
  }, data[index]);

  return data;
}, [] as number[][]);

const gammaRate = calculateGammaRate(totals);
const epsilonRate = calculateEpsilonRate(totals);

console.log('\n');
console.log(`The gamma rate is ${gammaRate}.`);
console.log(`The epsilon rate is ${epsilonRate}.`);
console.log(`The power consumption is ${gammaRate * epsilonRate}.`);
