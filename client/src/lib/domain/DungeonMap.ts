import { type Tilesheets } from "./Tilesheet";


export class MapState {
    private height: number;
    private width: number;
    private dungeon: Array<number>;
    private features: Map<string, number>; // 24-24, 127
    private actors: Map<string, number>; // but how do I identify actors?
    private players: Map<string, number>; // not sure, maybe have 
    private gfx: Tilesheets;
    private readonly tileSize: number;
    // actors be Map<string, Array<number>> ? 

    constructor(columns: number, rows: number, tileSheets: Tilesheets) {
        this.height = rows;
        this.width = columns;
        this.dungeon = Array(columns * rows)
        this.features = new Map<string, number>();
        this.actors = new Map<string, number>();
        this.players = new Map<string, number>();
        this.gfx = tileSheets;
        this.tileSize = 24;
    }

    addDungeon(x: number, y: number, tileIndex: number): void {
        this.dungeon[this.mapIdx(x, y)] = tileIndex;
    }

    // maybe make this replace it with a floor? 
    removeDungeon(x: number, y: number): void {
        delete this.dungeon[this.mapIdx(x, y)]
    }

    addFeature(x: number, y: number, tileIndex: number): void {
        this.features.set(this.key(x, y), tileIndex);
    }
    removeFeature(x: number, y: number): void {
        this.features.delete(this.key(x, y))
    }

    addActor(x: number, y: number, tileIndex: number) {
        this.addActorIdx(this.key(x, y), tileIndex);
    }

    private addActorIdx(idx: string, tileIndex: number): void {
        this.actors.set(idx, tileIndex);
    }

    removeActor(x: number, y: number): void {
        let idx: string = `${x}-${y}`
        this.removeActorIdx(this.key(x, y));
    }

    private removeActorIdx(idx: string): void {
        this.actors.delete(idx)
    }

    moveActor(fromX: number, fromY: number, toX: number, toY: number) {
        let idx: string = `${fromX}-${fromY}`
        let actorTile: number = this.actors[idx];
        this.removeActorIdx(idx)
        this.addActorIdx(this.key(toX, toY), actorTile)
    }

    private key(x: number, y: number): string {
        return `${x}-${y}`
    }

    private keyToNumber(s: string): [number, number] {
        let split = s.split("-");
        return [parseInt(split[0]), parseInt(split[1])]
    }

    mapIdx(x: number, y: number): number {
        return (y * this.width + x)
    }

    idxToCoords(idx: number): [number, number] {
        let x = idx % this.width
        let y = Math.floor(idx / this.width)
        return [x, y]
    }

    render(context: CanvasRenderingContext2D): void {

        // dungeon 

        this.dungeon.forEach((tile, i) => {
            let dim = this.gfx.dungeon.tiles[tile];
            let src = this.gfx.dungeon.src;
            let [canvasX, canvasY] = this.idxToCoords(i);
            context.drawImage(
                src,
                dim.sx,
                dim.sy,
                this.tileSize,
                this.tileSize,
                canvasX * this.tileSize,
                canvasY * this.tileSize,
                this.tileSize,
                this.tileSize
            );
        });

        // features 

        this.features.forEach((tile, key) => {
            let dim = this.gfx.feature.tiles[tile];
            let src = this.gfx.feature.src;
            let [canvasX, canvasY] = this.keyToNumber(key);
            context.drawImage(
                src,
                dim.sx,
                dim.sy,
                this.tileSize,
                this.tileSize,
                canvasX * this.tileSize,
                canvasY * this.tileSize,
                this.tileSize,
                this.tileSize
            );
        });


        // actors 

        // players ? 
    }


}