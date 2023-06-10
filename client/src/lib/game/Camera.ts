
export class Camera {
    leftX: number
    rightX: number
    topY: number
    bottomY: number
    hexWidth: number
    hexHeight: number

    // left-most visible tile is the players x coord minus half the cavnas size
    constructor(position: [number, number], hexWidth: number, hexHeight: number) {
        this.leftX = position[0] - hexWidth / 2;
        this.rightX = position[0] + hexWidth / 2;
        this.topY = position[1] - hexHeight / 2;
        this.bottomY = position[1] + hexHeight / 2;
        this.hexWidth = hexWidth;
        this.hexHeight = hexHeight;
    }

    onMove(position: [number, number]): void {
        this.leftX = position[0] - this.hexWidth / 2;
        this.rightX = position[0] + this.hexWidth / 2;
        this.topY = position[1] - this.hexHeight / 2;
        this.bottomY = position[1] + this.hexHeight / 2
    }
}