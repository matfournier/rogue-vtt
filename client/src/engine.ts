import { Terminal } from 'wglt';
import { Entity } from './entity';
import { GameMap } from './gamemap';

export class Engine {
  constructor(public entities: Entity[], public player: Entity, public gameMap: GameMap) {}

  handleEvents(term: Terminal) {
    const moveKey = term.getMovementKey();
    if (moveKey && this.gameMap.isWalkable(this.player.x + moveKey.x, this.player.y + moveKey.y)) {
      this.player.x += moveKey.x;
      this.player.y += moveKey.y;
    }
  }

  render(term: Terminal) {
    // Clear the screen
    term.clear();

    // Draw the game map
    this.gameMap.render(term);

    // Draw the entities
    for (const entity of this.entities) {
      term.drawChar(entity.x, entity.y, entity.char, entity.color);
    }
  }
}