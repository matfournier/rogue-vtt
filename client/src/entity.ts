import { Color } from 'wglt';

export class Entity {
  constructor(public x: number, public y: number, public char: string, public color: Color) {}

  move(dx: number, dy: number): void {
    this.x += dx;
    this.y += dy;
  }
}