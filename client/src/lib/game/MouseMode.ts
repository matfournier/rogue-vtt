
export type Mode = {
    major: string
    minor: string
    meta: string
}


// TODO going to need a SELECT mode and TARGET mode 
// this is so we can select a tile -> pick an enemy (or spawn a model when there is more than one enemy in a tile)
// and then MOVE them some how 
// also for spawning so we can spawn and move 
// maybe the NONE mode should really be SELECT? 


// when something IS selected, should be able to move it around w/ vim keys or arrow keys imho

export class MouseMode {

    private mode: Mode;

    constructor() {
        this.mode =
            { major: "NONE", minor: "DRAW", meta: "" }
    }

    get(): Mode {
        return this.mode
    }

    setMinorDraw(): void {
        this.mode.minor = "DRAW"
    }

    setMinorClear(): void {
        this.mode.minor = "CLEAR"
    }

    setMinorClearAll(): void {
        this.mode.minor = "CLEARALL"
    }

    setRange(): void {
        this.mode.major = "RANGE"
    }

    setMoving(): void {
        this.mode.major = "MOVING"
    }

    reset(): void {
        this.mode =
            { major: "NONE", minor: "DRAW", meta: "" }
    }
}