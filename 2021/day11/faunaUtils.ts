export class Octopus {
  constructor(
    public posx: number,
    public posy: number,
    public energy: number
  ) {}
}

// const printGrid = (octopii: Octopus[][]) => {
//   return octopii.map((row) => row.map((n) => n.energy).join('')).join('\n');
// };

export const getNeighbors = (octopii: Octopus[][], o: Octopus) =>
  [
    octopii[o.posy - 1]?.[o.posx - 1],
    octopii[o.posy - 1]?.[o.posx],
    octopii[o.posy - 1]?.[o.posx + 1],
    octopii[o.posy]?.[o.posx - 1],
    octopii[o.posy]?.[o.posx + 1],
    octopii[o.posy + 1]?.[o.posx - 1],
    octopii[o.posy + 1]?.[o.posx],
    octopii[o.posy + 1]?.[o.posx + 1],
  ].filter((o) => o !== undefined);

export const increaseEnergies = (octopii: Octopus[][]) => {
  for (let y = 0; y < octopii.length; y++) {
    for (let x = 0; x < octopii[y].length; x++) {
      octopii[y][x].energy += 1;
    }
  }
};

export const getReadyToFlashOctopii = (octopii: Octopus[][]) => {
  const readyToFlash = [];

  for (let y = 0; y < octopii.length; y++) {
    for (let x = 0; x < octopii[y].length; x++) {
      if (octopii[y][x].energy > 9) {
        readyToFlash.push(octopii[y][x]);
      }
    }
  }

  return readyToFlash;
};

export const flash = (octopii: Octopus[][], readyToFlash: Octopus[]) => {
  return readyToFlash.map((o) => {
    const flashQueue = [o];
    let currentFlashes = 0;

    while (flashQueue.length > 0) {
      const current = flashQueue.shift()!;
      currentFlashes += 1;
      const neighbors = getNeighbors(octopii, current);

      neighbors.forEach((n) => {
        if (n.energy <= 9) {
          n.energy += 1;
          if (n.energy > 9) {
            flashQueue.push(n);
          }
        }
      });
    }

    return currentFlashes;
  });
};

export const resetEnergies = (octopii: Octopus[][]) => {
  for (let y = 0; y < octopii.length; y++) {
    for (let x = 0; x < octopii[y].length; x++) {
      octopii[y][x].energy =
        octopii[y][x].energy > 9 ? 0 : octopii[y][x].energy;
    }
  }
};
