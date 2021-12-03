// Day 2 part 1
// https://adventofcode.com/2021/day/2
// input: day2/input.txt

const coords = {
  horizontal: 0,
  depth: 0,
};

const submarineCourse = (await Deno.readTextFile('day2/input.txt')).split('\n');

const finalCoords = submarineCourse.reduce((currrentCoods, instr) => {
  const [direction, distance] = instr.split(' ');
  switch (direction.toLowerCase()) {
    case 'forward':
      currrentCoods.horizontal += Number(distance);
      break;
    case 'down':
      currrentCoods.depth += Number(distance);
      break;
    case 'up':
      currrentCoods.depth -= Number(distance);
      break;
  }
  return currrentCoods;
}, coords);

console.log(
  `Final coords are ${finalCoords.horizontal} units forward and ${finalCoords.depth} units down.`
);
console.log(
  `Coordinate product is ${finalCoords.horizontal * finalCoords.depth}.`
);
