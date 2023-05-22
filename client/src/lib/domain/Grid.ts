
export class Grid {
    // TODO incorporate camera. 
    // canvas to (x,y) hex 
    static getTileCoords(x: number, y: number): [number, number] {
        return [Math.floor(x / 24), Math.floor(y / 24)];
    }
}
