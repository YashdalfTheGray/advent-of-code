// Day 2 part 1
// https://adventofcode.com/2021/day/2
// input: day2/input.txt

const coordsWithAim = {
  horizontal: 0,
  depth: 0,
  aim: 0,
};

const updatedSubmarineCourse = (
  await Deno.readTextFile('day2/input.txt')
).split('\n');

const finalCoordsWithAim = updatedSubmarineCourse.reduce(
  (currrentCoods, instr) => {
    const [direction, distance] = instr.split(' ');
    switch (direction.toLowerCase()) {
      case 'forward':
        currrentCoods.horizontal += Number(distance);
        currrentCoods.depth += Number(distance) * currrentCoods.aim;
        break;
      case 'down':
        currrentCoods.aim += Number(distance);
        break;
      case 'up':
        currrentCoods.aim -= Number(distance);
        break;
    }
    return currrentCoods;
  },
  coordsWithAim
);

console.log(
  `Final coords are ${finalCoordsWithAim.horizontal} units forward and ${finalCoordsWithAim.depth} units down. The aim is ${finalCoordsWithAim.aim}.`
);
console.log(
  `Coordinate product is ${
    finalCoordsWithAim.horizontal * finalCoordsWithAim.depth
  }.`
);
