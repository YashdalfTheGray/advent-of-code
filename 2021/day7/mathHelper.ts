export const calculateTotalFuel = (
  crabs: number[],
  toPosition: number,
  fuelCostFunction: (n: number) => number
) =>
  crabs.reduce(
    (groupFuel, crab) =>
      groupFuel + fuelCostFunction(Math.abs(toPosition - crab)),
    0
  );

export const median = (numbers: number[]): number => {
  const middle = Math.floor(numbers.length / 2);
  return numbers.length % 2 !== 0
    ? numbers[middle]
    : (numbers[middle - 1] + numbers[middle]) / 2;
};

export const mean = (numbers: number[]): number =>
  numbers.reduce((sum, number) => sum + number, 0) / numbers.length;
