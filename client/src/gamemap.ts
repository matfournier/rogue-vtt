import { Console, Terminal } from 'wglt';
import { floor, wall } from './tiles';

export class GameMap {
  private console: Console;
  constructor(public width: number, public height: number) {
    this.console = new Console(width, height);

    // Fill the map with floor tiles.
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        this.console.drawCell(x, y, floor);
        this.console.setBlocked(x, y, false);
      }
    }

    // Create a small section of wall tiles.
    for (let x = 30; x < 33; x++) {
      this.console.drawCell(x, 20, wall);
      this.console.setBlocked(x, 20, true);
    }
  }

  isWalkable(x: number, y: number): boolean {
    return !this.console.isBlocked(x, y);
  }

  render(term: Terminal): void {
    term.drawConsole(0, 0, this.console, 0, 0, this.width, this.height);
  }
}