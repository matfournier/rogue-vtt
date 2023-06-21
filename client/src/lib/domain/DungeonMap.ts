import type { Camera } from "../game/Camera";
import type { Entity } from "./EntityRenderer";
import { type Tilesheets } from "./Tilesheet";

export class MapState {
    private camera: Camera;
    private height: number;
    private width: number;
    private dungeon: Array<number>;
    private features: Map<string, number>; // 24-24, 127

    private gfx: Tilesheets;
    private readonly tileSize: number;

    constructor(columns: number, rows: number, tileSheets: Tilesheets, camera: Camera) {
        this.height = rows;
        this.width = columns;
        this.dungeon = Array(columns * rows)
        this.features = new Map<string, number>();
        this.camera = camera;

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
        let res = (y * this.width + x)
        return (y * this.width + x)
    }

    idxToCoords(idx: number): [number, number] {
        let x = idx % this.width
        let y = Math.floor(idx / this.width)
        return [x, y]
    }

    render(context: CanvasRenderingContext2D, x: number, y: number): void {
        // dungeon 
        let tile = this.dungeon[this.mapIdx(x, y)];
        let tileSprite = this.gfx.dungeon.tiles[tile]
        if (tile !== undefined) {
            let src = this.gfx.dungeon.src;
            context.drawImage(
                src,
                tileSprite.sx,
                tileSprite.sy,
                this.tileSize,
                this.tileSize,
                (x - this.camera.leftX) * this.tileSize,
                (y - this.camera.topY) * this.tileSize,
                this.tileSize,
                this.tileSize
            );
        }
        let key = this.key(x, y);
        let featureTile = this.features.get(key)
        if (featureTile !== undefined) {
            let tileSprite = this.gfx.feature.tiles[featureTile]
            context.drawImage(
                this.gfx.feature.src,
                tileSprite.sx,
                tileSprite.sy,
                this.tileSize,
                this.tileSize,
                (x - this.camera.leftX) * this.tileSize,
                (y - this.camera.topY) * this.tileSize,
                this.tileSize,
                this.tileSize
            );

        }


    }


}