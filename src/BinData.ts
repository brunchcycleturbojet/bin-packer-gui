export class Bin {
  id = "";
  width = 0;
  height = 0;
  depth = 0;

  items: Array<Item> = [];

  constructor(id: string, w: number, h: number, d: number) {
    this.id = id;
    this.width = w;
    this.height = h;
    this.depth = d;
  }
}

export type Item = {
  id: number,     // Unique ID
  name: string,   // Aa-Zz, 0-9 freely customisable name
  x: number,
  y: number,
  z: number,
  width: number,
  height: number,
  depth: number,
  rotate_x: number,
  rotate_y: number,
  rotate_z: number,
}