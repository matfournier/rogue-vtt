
export class Logic {
//Utility for getting coordinates of mouse click
//We get these Down (x) and Right (y) but we need to invert them for our coordinate system 
static getTileCoords(x: number, y: number): [number, number] {
    return [Math.floor(y / 32), Math.floor(x / 32)];
}


static tileIndex(x: number, y: number): number { 
    return x * y
}

}