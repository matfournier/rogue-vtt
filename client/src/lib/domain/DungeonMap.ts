import type { Entity } from "./EntityRenderer";
import { type Tilesheets } from "./Tilesheet";


// should we have a second tileset where we invert the grid border so people can see the grid more easily
// or should we be able to draw it using the canvas a series of lines? 
//   ^-- try this. 

export class MapState {
    private height: number;
    private width: number;
    private dungeon: Array<number>;
    private features: Map<string, number>; // 24-24, 127

    private gfx: Tilesheets;
    private readonly tileSize: number;
    // actors be Map<string, Array<number>> ? 

    constructor(columns: number, rows: number, tileSheets: Tilesheets) {
        this.height = rows;
        this.width = columns;
        this.dungeon = Array(columns * rows)
        this.features = new Map<string, number>();

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