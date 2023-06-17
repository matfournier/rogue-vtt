
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
        this.leftX = Math.max(0, position[0] - hexWidth);
        this.rightX = Math.min(maxWidth, position[0] + hexWidth);
        this.topY = Math.max(0, position[1] - hexHeight);
        this.bottomY = Math.min(maxHeight, position[1] + hexHeight);
        this.hexWidth = hexWidth;
        this.hexHeight = hexHeight;
        this.maxWidth = maxWidth;
        this.maxHeight = maxHeight;
        console.log(`ll: ${this.leftX}, r: ${this.rightX}, t: ${this.topY}, b: ${this.bottomY}`)
    }

    onMove(position: [number, number]): void {
        console.log(` pre top: ${this.topY} bottom: ${this.bottomY} pos: ${position[0]} ${position}`)
        this.leftX = position[0] - this.hexWidth / 2
        this.rightX = position[0] + this.hexWidth / 2;
        this.topY = position[1] - this.hexHeight / 2;
        this.bottomY = position[1] + this.hexHeight / 2;
        this.snap();
        console.log(` post top: ${this.topY} bottom: ${this.bottomY}`)

    }

    private snap(): void {
        if (this.leftX < 0) {
            this.leftX = 0;
            this.rightX = this.leftX + this.hexWidth;
        } else if (this.rightX > this.maxWidth) {
            this.rightX = this.maxWidth;
            this.leftX = this.rightX - this.hexWidth;
        }
        if (this.topY < 0) {
            this.topY = 0;
            this.bottomY = this.topY + this.hexHeight;
        } else if (this.bottomY > this.maxHeight) {
            this.bottomY = this.maxHeight;
            this.topY = this.bottomY - this.hexHeight;
        }
    }

    down(): void {
        this.topY += 1;
        this.bottomY += 1;
        this.snap;
    }

    up(): void {
        this.topY -= 1;
        this.bottomY -= 1;
        this.snap();
    }
    right(): void {
        this.leftX += 1;
        this.rightX += 1;
        this.snap();
    }
    left(): void {
        this.leftX -= 1;
        this.rightX -= 1;
        this.snap();
    }

    convertXY(xy: [number, number]): [number, number] {
        return [xy[0] - this.leftX, xy[1] - this.topY]
    }
}