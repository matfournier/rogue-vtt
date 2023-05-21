
export class Logic {

static getTileCoords(x: number, y: number): [number, number] {
    return [Math.floor(x / 24), Math.floor(y / 24)];
}



// my array indexing is all wrong 
// static tileIndex(x: number, y: number): number { 
//     return (x+1) * y
// }


}