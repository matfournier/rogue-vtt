import type { Entity } from "../domain/EntityRenderer";

export type Mode = {
    major: string
    minor: string
    meta: string
}


// refactor why not make this functional. 

// mousemode gives three functions: 
// onset 
// onstart
// onend 


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

    setMovingStart(): void {
        this.mode.major = "MOVING"
        this.mode.minor = "SELECT"
    }

    setSelection() {
        this.mode.major = "SELECTION"
        this.mode.minor = "NOTARGET"
    }
    setSelectionSelected(entity: Entity) {
        if (this.mode.major === "SELECTION") {
            this.mode.minor = "TARGET"
            this.mode.meta = entity.id
        }
    }

    reset(): void {
        this.mode =
            { major: "NONE", minor: "DRAW", meta: "" }
    }
}