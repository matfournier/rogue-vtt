import { Colors, Terminal } from 'wglt';
import { Engine } from './engine';
import { Entity } from './entity';
import { GameMap } from './gamemap';

const SCREEN_WIDTH = 80;
const SCREEN_HEIGHT = 45;

const MAP_WIDTH = 80;
const MAP_HEIGHT = 40;

const term = new Terminal(document.querySelector('canvas') as HTMLCanvasElement, SCREEN_WIDTH, SCREEN_HEIGHT);
const player = new Entity(40, 20, '@', Colors.WHITE);
const npc = new Entity(35, 20, '@', Colors.YELLOW);
const entities = [player, npc];
const gameMap = new GameMap(MAP_WIDTH, MAP_HEIGHT);
const engine = new Engine(entities, player, gameMap);

term.update = () => {
  engine.handleEvents(term);
  engine.render(term);
};