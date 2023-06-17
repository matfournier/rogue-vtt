import type { Entity, EntityType } from "../domain/EntityRenderer"
import type { Bounds } from "../domain/Grid"
import type { Icons } from "../domain/Tilesheet"
import type { Camera } from "./Camera"
import { MouseMode } from "./MouseMode"
import { SquareCounter } from "../domain/Grid"
import type { SelectedTile } from "../domain/SelectedTile"

export enum EventType {
    DISPLAY,
    GAME
}

export enum ActionType {
    TilePlaced,
    TileRemoved,
    Fill,
    Clear,
    AddToken,
    TokenDescription,
    RemoveToken,
    RenameToken,
    MoveToken
}

export enum UActionType {
    Reset,
    IgnoreKeyboard
}

export type TilePlacedAction = {
    kind: ActionType.TilePlaced
    xy: [number, number]
    tileset: number // 0 for dungeon, 1 for feature 
    idx: number
}

export type TileRemovedAction = {
    kind: ActionType.TileRemoved
    xy: [number, number]
    layer: number // 0 for dungeon, 1 for feature, 2 for all 
}

export type FillAction = {
    kind: ActionType.Fill
    bounds: Bounds
    tileset: number // 0 for dungeon, 1 for feature
    idx: number
}

export type ClearAction = {
    kind: ActionType.Clear
    bounds: Bounds
    layer: number // 0 for dungeon, 1 for feature, 2 for all 
}

export type AddTokenAction = {
    kind: ActionType.AddToken
    entity: Entity
}

export type TokenDescriptionAction = {
    kind: ActionType.TokenDescription
    side: EntityType
    desc: string
}

export type RemoveTokenAction = {
    kind: ActionType.RemoveToken
    entity: Entity
    xy?: [number, number]
}

export type MoveTokenAction = {
    kind: ActionType.MoveToken
    entity: Entity
    from: [number, number]
    to: [number, number]
}

export type Action = TilePlacedAction | TileRemovedAction | FillAction |
    ClearAction | AddTokenAction | TokenDescriptionAction | RemoveTokenAction | MoveTokenAction


export type ResetUAction = {
    kind: UActionType.Reset
}

export type IgnoreKeyboard = {
    kind: UActionType.IgnoreKeyboard
}

export type UAction = ResetUAction

export type DisplayEvent = {
    type: EventType.DISPLAY
    action: UAction
}

export type GameEvent = {
    type: EventType.GAME
    action: Action
}

export type Event = DisplayEvent | GameEvent

/*


Display Events 

Camera mova event <- not sure?
Draw yellow cursor event  // replaced by render
Draw red cursor event // replace by render 
Change mode (explore -> move perhaps?)
ConvertHandler -> e.g. ViewHandler -> MoveHandler
Reset event



*/

export interface InteractionHandler {
    onClick?(xy: [number, number]): void
    onMove?(xy: [number, number]): Array<Event>
    onEnd?(xy: [number, number]): Array<Event>
    onKeyDown(e: KeyboardEvent): Array<Event>
    onKeyUp(e: KeyboardEvent): Array<Event>
    onKeyPressed(e: KeyboardEvent): Array<Event>
    render(context: CanvasRenderingContext2D): void
    update(selectedTile: SelectedTile): void
}

export class ViewHandler implements InteractionHandler {
    camera: Camera;
    cursor: [number, number]; // where is this coming from! 
    icons: Icons;

    constructor(camera: Camera, icons: Icons, cursor: [number, number]) {
        this.camera = camera
        this.icons = icons;
        this.cursor = cursor
    }

    onClick?(xy: [number, number]): void {
        this.camera.onMove(xy);
    }
    onMove?(xy: [number, number]): Event[] {
        // should cover popup/tooltip type things when you hover over things 
        // otherwise just the cursor
        this.cursor = xy;
        return [];
    }
    onEnd?(xy: [number, number]): Event[] {
        return [];
    }
    onKeyDown(e: KeyboardEvent): Event[] {
        return [];
    }
    onKeyUp(e: KeyboardEvent): Event[] {
        // todo vim keys for moving around 
        // TODO updating cursor after moving (should emit a CursorUpdate event or something?)
        switch (e.code) {
            case "ArrowDown": // down arrow
                this.camera.down();
                break;
            case "ArrowUp": // up arrow
                this.camera.up();
                break;
            case "ArrowLeft": // left arrow
                this.camera.left();
                break;
            case "ArrowRight": // right arrow
                this.camera.right();
                break;
        }
        return [];
    }
    onKeyPressed(e: KeyboardEvent): Event[] {
        return [];
    }
    render(context: CanvasRenderingContext2D): void {
        this.icons.renderCursor(context, this.cursor);
        // can render cursor and popups in this thing 
    }

    update(selectedTile: SelectedTile): void { }


}

export class DrawHandler implements InteractionHandler {

    mouseMode: MouseMode;
    clickBounds: SquareCounter;
    selectedTile: SelectedTile;


    constructor(selectedTile: SelectedTile) {
        this.mouseMode = new MouseMode();
        this.selectedTile = selectedTile;
    }

    onClick?(xy: [number, number]): void {
        if (this.mouseMode.get().major !== "SELECTION") {
            this.clickBounds = new SquareCounter(xy)
            if (this.mouseMode.get().major === "NONE") {
                this.mouseMode.setRange();
            }
        }
    }
    onMove?(xy: [number, number]): Event[] {
        throw new Error("Method not implemented.")
    }
    onEnd?(xy: [number, number]): Event[] {
        let mode = this.mouseMode.get();
        if (mode.major === "RANGE") {
            let bounds = this.clickBounds.bounds();
            if (mode.minor === "DRAW") {
                let action: FillAction = {
                    kind: ActionType.Fill,
                    bounds: bounds,
                    tileset: this.getTileLayer(),
                    idx: this.selectedTile.idx
                }
                return new Array({ type: EventType.GAME, action: action })
                // return a bunch of create tiles events 
            } else if (mode.minor === "CLEAR" || mode.minor === "CLEARALL") {
                // return a bunch of clear tile events 
                return [];
            }
        }
    }

    onKeyDown(e: KeyboardEvent): Event[] {
        throw new Error("Method not implemented.")
    }
    onKeyUp(e: KeyboardEvent): Event[] {
        throw new Error("Method not implemented.")
    }
    onKeyPressed(e: KeyboardEvent): Event[] {
        throw new Error("Method not implemented.")
    }
    render(context: CanvasRenderingContext2D): void { }

    getTileLayer(): number {
        if (this.selectedTile.sheet === "dungeon") {
            return 0;
        } else {
            return 1;
        }
    }
    update(selectedTile: SelectedTile): void {
        this.selectedTile = selectedTile;
    }

}

export class MoveHandler implements InteractionHandler {
    onClick?(xy: [number, number]): Event[] {
        throw new Error("Method not implemented.")
    }
    onMove?(xy: [number, number]): Event[] {
        throw new Error("Method not implemented.")
    }
    onEnd?(xy: [number, number]): Event[] {
        throw new Error("Method not implemented.")
    }
    onKeyDown(e: KeyboardEvent): Event[] {
        throw new Error("Method not implemented.")
    }
    onKeyUp(e: KeyboardEvent): Event[] {
        throw new Error("Method not implemented.")
    }
    onKeyPressed(e: KeyboardEvent): Event[] {
        throw new Error("Method not implemented.")
    }

    render(context: CanvasRenderingContext2D): void { }

    update(selectedTile: SelectedTile): void { }

}