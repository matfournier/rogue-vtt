
// see https://github.com/mozdevs/gamedev-js-tiles/blob/gh-pages/square/scroll.js

export class Camera {
    px: number;
    py: number;
    width: number;
    height: number;
    maxX: number;

    constructor(px: number, py: number, width: number, height: number) {
        this.px = px;
        this.py = py;
        this.width = width;
        this.height = height; 
    }
}
