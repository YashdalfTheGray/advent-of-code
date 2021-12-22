export default class Point {
  public static encode(x: number, y: number, z: number): string {
    return `(${x},${y},${z})`;
  }

  public static decode(point: string): [number, number, number] {
    const [x, y, z] = point.slice(1, -1).split(',').map(Number);
    return [x, y, z];
  }
}
