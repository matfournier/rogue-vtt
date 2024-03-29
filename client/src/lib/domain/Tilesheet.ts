import floorWallPng from '../../assets/vttfloorsandwalls24x24.png'
import featurePng from '../../assets/features24x24.png'
import iconPng from '../../assets/vtticon24x24.png'


export type Tilesheets = {
    dungeon: Tilesheet // 0
    feature: Tilesheet // 1
    icon: Icons
}

export class Tilesheet {
    readonly numTiles: number;
    readonly src: HTMLImageElement;
    readonly tilesPerRow: number = 32;
    readonly tileSize: number = 24;

    tiles: Array<{ sx: number, sy: number }> = new Array()

    constructor(numTiles: number, src: HTMLImageElement) {
        this.numTiles = numTiles;
        this.src = src;

        for (let i = 0; i < numTiles; i++) {
            let sx: number = (i % this.tilesPerRow) * 24
            let sy: number = Math.floor(i / this.tilesPerRow) * 24
            this.tiles.push({ sx: sx, sy: sy });
        }
    }

    render(context: CanvasRenderingContext2D): void {
        this.tiles.forEach((tile, i) => {
            // let dim = this.gfx.dungeon.tiles[tile]
            // let [canvasX, canvasY] = this.idxToCoords(i)
            context.drawImage(
                this.src,
                tile.sx,
                tile.sy,
                this.tileSize,
                this.tileSize,
                tile.sx,
                tile.sy,
                this.tileSize,
                this.tileSize
            );
        })
    }
}

export class Icons {
    readonly sheet: Tilesheet

    private cursorIdx: number = 0;
    private tileSize: number = 24;

    constructor(ts: Tilesheet) {
        this.sheet = ts;
    }

    cursor(): { sx: number, sy: number } {
        return { "sx": this.cursorIdx * 24, "sy": 0 }
    }

    renderCursor(context: CanvasRenderingContext2D, xy: [number, number]): void {
        context.drawImage(
            this.sheet.src,
            0,
            0,
            this.tileSize,
            this.tileSize,
            xy[0] * this.tileSize,
            xy[1] * this.tileSize,
            this.tileSize,
            this.tileSize
        );
    }

    renderSelectionCursor(context: CanvasRenderingContext2D, xy: [number, number]): void {
        context.drawImage(
            this.sheet.src,
            24,
            0,
            this.tileSize,
            this.tileSize,
            xy[0] * this.tileSize,
            xy[1] * this.tileSize,
            this.tileSize,
            this.tileSize
        );
    }
}

export async function tilesheets(): Promise<Tilesheets> {
    let imgs = [{
        "name": "dungeon",
        "src": floorWallPng,
        "numTiles": 169
    },
    {
        "name": "feature",
        "src": featurePng,
        "numTiles": 64
    },
    {
        "name": "icons",
        "src": iconPng,
        "numTiles": 7
    }
    ]
    return loadImages(imgs)
}


async function loadImages(imageUrlArray: Array<{ "name": string, "src": string, numTiles: number }>): Promise<Tilesheets> {
    const promiseArray: Array<Promise<{ "name": string, ts: Tilesheet }>> = []; // create an array for promises
    for (let imageUrl of imageUrlArray) {
        promiseArray.push(new Promise(resolve => {
            const img = new Image();
            img.onload = function () {
                let ts = new Tilesheet(imageUrl.numTiles, img)
                resolve({ "name": imageUrl.name, "ts": ts });
            };

            img.src = imageUrl.src;
        }));
    }

    let sheets = await Promise.all(promiseArray); // wait for all the images to be loaded
    console.log("all images loaded");
    return {
        "dungeon": sheets.find(ts => ts.name === "dungeon").ts,
        "feature": sheets.find(ts => ts.name === "feature").ts,
        "icon": new Icons(sheets.find(ts => ts.name === "icons").ts)
    }
}