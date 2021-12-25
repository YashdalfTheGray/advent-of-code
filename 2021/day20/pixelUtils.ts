export const getSquareOfPixels = <T>(
  grid: T[][],
  posx: number,
  posy: number,
  defaultValue: T
): T[] =>
  [
    grid[posy - 1]?.[posx - 1],
    grid[posy - 1]?.[posx],
    grid[posy - 1]?.[posx + 1],
    grid[posy]?.[posx - 1],
    grid[posy]?.[posx],
    grid[posy]?.[posx + 1],
    grid[posy + 1]?.[posx - 1],
    grid[posy + 1]?.[posx],
    grid[posy + 1]?.[posx + 1],
  ].map((v) => (v !== undefined ? v : defaultValue));

export const pixelsToNumber = (pixels: string[]) =>
  parseInt(pixels.map((p) => (p === '#' ? 1 : 0)).join(''), 2);

export const addBorder = (image: string[][], border: string) => [
  Array(image[0].length + 2).fill(border),
  ...image.map((r) => [border, ...r, border]),
  Array(image[0].length + 2).fill(border),
];

export const determineNewDefaultValue = (
  imageProcessingMap: string[],
  previousDefaultValue: string
): string => {
  if (
    imageProcessingMap[0] === '#' &&
    imageProcessingMap[imageProcessingMap.length - 1] === '.'
  ) {
    return previousDefaultValue === '#' ? '.' : '#';
  } else {
    return '.';
  }
};

export const printImage = (image: string[][]) =>
  image.map((r) => r.join('')).join('\n');
