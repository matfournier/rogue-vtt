
export class Camera {
    leftX: number
    rightX: number
    topY: number
    bottomY: number
    hexWidth: number
    hexHeight: number
    maxWidth: number
    maxHeight: number

    // left-most visible tile is the players x coord minus half the cavnas size
    constructor(position: [number, number], hexWidth: number, hexHeight: number, maxWidth: number, maxHeight: number) {
        this.leftX = Math.max(0, position[0] - hexWidth / 2);
        this.rightX = Math.min(maxWidth, position[0] + hexWidth / 2);
        this.topY = Math.max(0, position[1] - hexHeight / 2);
        this.bottomY = Math.min(maxHeight, position[1] + hexHeight / 2);
        this.hexWidth = hexWidth;
        this.hexHeight = hexHeight;
        this.maxWidth = maxWidth;
        this.maxHeight = maxHeight;
        console.log(`ll: ${this.leftX}, r: ${this.rightX}, t: ${this.topY}, b: ${this.bottomY}`)
    }

    onMove(position: [number, number]): void {
        this.leftX = Math.max(0, position[0] - this.hexWidth / 2)
        this.rightX = Math.min(this.maxWidth, position[0] + this.hexWidth / 2);
        this.topY = Math.max(0, position[1] - this.hexHeight / 2);
        this.bottomY = Math.min(this.maxHeight, position[1] + this.hexHeight / 2)
    }
}