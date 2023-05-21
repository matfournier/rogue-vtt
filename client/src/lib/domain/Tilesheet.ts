import floorWallPng from '../../assets/vttfloorsandwalls24x24.png'
import featurePng from '../../assets/vttfeat24x24.png'
import iconPng from '../../assets/vtticon24x24.png'


export type Tilesheets = {
    dungeon: Tilesheet
    feature: Tilesheet
    icon: Icons
}

export class Tilesheet {
    readonly numTiles: number;
    readonly src: HTMLImageElement;
    readonly tilesPerRow: number = 32;

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
}

export class Icons {
    readonly sheet: Tilesheet

    private cursorIdx: number = 0

    constructor(ts: Tilesheet) {
        this.sheet = ts
    }

    cursor(): { sx: number, sy: number } {
        return { "sx": this.cursorIdx * 24, "sy": 0 }
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
        "numTiles": 72
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
            // if you don't need to do anything when the image loads,
            // then you can just write img.onload = resolve;

            img.onload = function () {
                // do stuff with the image if necessary
                let ts = new Tilesheet(imageUrl.numTiles, img)
                // resolve the promise, indicating that the image has been loaded
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