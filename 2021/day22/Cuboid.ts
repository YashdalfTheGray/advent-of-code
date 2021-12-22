import { assert } from 'https://deno.land/std@0.118.0/testing/asserts.ts';

export type Range1D = { min: number; max: number };
export type CuboidBounds = [
  xmin: number,
  xmax: number,
  ymin: number,
  ymax: number,
  zmin: number,
  zmax: number
];

export default class Cuboid {
  public state: boolean;
  private x: Range1D;
  private y: Range1D;
  private z: Range1D;

  constructor(input: string) {
    const splitInput = input.split(' ');
    this.state = splitInput[0] === 'on';
    const [xrangeStr, yrangeStr, zrangeStr] = splitInput[1].split(',');

    this.x = this.translateRangeStringIntoRange1D(xrangeStr);
    this.y = this.translateRangeStringIntoRange1D(yrangeStr);
    this.z = this.translateRangeStringIntoRange1D(zrangeStr);
  }

  public get bounds(): CuboidBounds {
    return [
      this.x.min,
      this.x.max,
      this.y.min,
      this.y.max,
      this.z.min,
      this.z.max,
    ];
  }

  *[Symbol.iterator]() {
    for (let x = this.x.min; x <= this.x.max; x++) {
      for (let y = this.y.min; y <= this.y.max; y++) {
        for (let z = this.z.min; z <= this.z.max; z++) {
          yield [x, y, z];
        }
      }
    }
  }

  private translateRangeStringIntoRange1D(range: string): Range1D {
    const splitRange = range.split('=');
    const [min, max] = splitRange[1].split('..').map(Number);
    assert(min <= max, 'range min is higher than range max');
    return { min, max };
  }
}
