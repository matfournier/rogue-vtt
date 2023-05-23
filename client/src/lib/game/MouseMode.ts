
export type Mode = {
    major: string
    minor: string
}

export class MouseMode {

    private mode: Mode;

    constructor() {
        this.mode =
            { major: "NONE", minor: "DRAW" }
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

    setSelection(): void {
        this.mode.major = "SELECTION"
    }

    setMoving(): void {
        this.mode.major = "MOVING"
    }

    reset(): void {
        this.mode =
            { major: "NONE", minor: "DRAW" }
    }
}