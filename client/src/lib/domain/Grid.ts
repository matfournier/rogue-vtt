
export class Grid {
    // canvas to (x,y) hex 
    static getTileCoords(x: number, y: number): [number, number] {
        return [Math.floor(x / 24), Math.floor(y / 24)];
    }
}

export type Bounds = {
    x: [number, number]
    y: [number, number]
}

export class SquareCounter {
    private minX: number
    private maxX: number
    private minY: number
    private maxY: number
    private squareTiles: Array<[number, number]>

    constructor(initial: [number, number]) {
        this.minX = initial[0];
        this.maxX = initial[0];
        this.minY = initial[1];
        this.maxY = initial[1];
        this.squareTiles = new Array();
        this.squareTiles.push(initial);

    }

    public count(xy: [number, number]): void {
        this.minX = Math.min(this.minX, xy[0]);
        this.maxX = Math.max(this.maxX, xy[0])
        this.minY = Math.min(this.minY, xy[1]);
        this.maxY = Math.max(this.maxY, xy[1]);
        console.log(this)
    }

    public lim(xy: [number, number]): void {
        this.maxX = xy[0]
        this.maxY = xy[1]

    }

    bounds(): Bounds {
        const xmin = Math.min(this.minX, this.maxX);
        const xmax = Math.max(this.minX, this.maxX);
        const ymin = Math.min(this.minY, this.maxY);
        const ymax = Math.max(this.minY, this.maxY);

        return { x: [xmin, xmax], y: [ymin, ymax] }
    }

    tiles(): Array<[number, number]> {
        for (let i = this.minX; i <= this.maxX; i++) {
            for (let j = this.minY; j <= this.maxY; j++) {
                this.squareTiles.push([i, j]);
            }
        }
        return this.squareTiles
    }
    tilesLim(): Array<[number, number]> {
        const xmin = Math.min(this.minX, this.maxX);
        const xmax = Math.max(this.minX, this.maxX);
        const ymin = Math.min(this.minY, this.maxY);
        const ymax = Math.max(this.minY, this.maxY);

        let res: Array<[number, number]> = new Array()

        for (let i = xmin; i <= xmax; i++) {
            for (let j = ymin; j <= ymax; j++) {
                res.push([i, j]);
            }
        }

        return res


    }
}