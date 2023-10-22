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
    // TokenDescription,
    RemoveToken,
    RenameToken,
    MoveToken,
    Init,
    Load,
}

export enum UActionType {
    Reset,
    IgnoreKeyboard,
    RestoreKeyboard,
    PopupDungeon,
    PopupFeature,
    ChangeInteraction,
    PlaceToken,
    Ignore,
    MoveEntityStart
}

export enum InteractionType {
    View,
    Draw,
    Place
}

// todo.
export type LoadAction = {
    xy: [number, number]
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
    entity: Entity,
}


// We will want some way to update an entity description
// cross that bridge when it comes 
// export type TokenDescriptionAction = {
//     kind: ActionType.TokenDescription
//     side: EntityType
//     token: string
//     desc: string
// }

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

// Only generated server side
export type InitAction = {
    kind: ActionType.Init
    xy: [number, number]
    id: String
}

// export type Tile = {
//     x: number,
//     y: number,
//     idx: number
// }

// export type Level = {
//     description: String,
//     id: String,
//     dimension: [number, number],
//     tiles: Array<Tile>,
//     features: Array<Tile>
// }

// export type Entities = {

// }
// // export type LoadAction = {
// //     kind: ActionType.Load

// // }

export function toInitAction(from: any): InitAction {
    return from as InitAction
}

export type Action = TilePlacedAction | TileRemovedAction | FillAction |
    ClearAction | AddTokenAction | RemoveTokenAction | MoveTokenAction | InitAction


export type ResetUAction = {
    kind: UActionType.Reset
}

export type IgnoreKeyboard = {
    kind: UActionType.IgnoreKeyboard
}

export type RestoreKeyboard = {
    kind: UActionType.RestoreKeyboard
}

export type PopupDungeon = {
    kind: UActionType.PopupDungeon
}

export type PopupFeature = {
    kind: UActionType.PopupFeature
}

export type ChangeInteraction = {
    kind: UActionType.ChangeInteraction
    value: InteractionType
}

export type MoveEntityStart = {
    kind: UActionType.MoveEntityStart
    entity: Entity
    xy: [number, number]
}

export type PlaceTokenPopup = {
    kind: UActionType.PlaceToken
    xy: [number, number]
}

export type IgnoreAction = {
    kind: UActionType.Ignore
}

export type UAction = ResetUAction | IgnoreKeyboard | RestoreKeyboard | PopupDungeon | PopupFeature |
    ChangeInteraction | PlaceTokenPopup | IgnoreAction | MoveEntityStart


export type DisplayEvent = {
    type: EventType.DISPLAY
    action: UAction
}

export type GameEvent = {
    type: EventType.GAME
    action: Action
}

export type Event = DisplayEvent | GameEvent

// all xy coordinates are MAP coordinators, not canvas coordinatoes 
export interface InteractionHandler {
    onClick?(xy: [number, number]): void
    onMove?(xy: [number, number]): Array<Event>
    onEnd?(xy: [number, number]): Array<Event>
    onLeave?(xy: [number, number]): Array<Event>
    onKeyDown(e: KeyboardEvent): Array<Event>
    onKeyUp(e: KeyboardEvent): Array<Event>
    onKeyPressed(e: KeyboardEvent): Array<Event>
    render(context: CanvasRenderingContext2D): void
    update(selectedTile: SelectedTile, entity?: Entity): void
}

export class ViewHandler implements InteractionHandler {
    camera: Camera;
    cursor: [number, number];
    icons: Icons;

    constructor(camera: Camera, icons: Icons, cursor: [number, number]) {
        this.camera = camera;
        this.icons = icons;
        this.cursor = cursor;
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
    onLeave?(xy: [number, number]): Array<Event> {
        return [];
    }

    onKeyDown(e: KeyboardEvent): Event[] {
        return [];
    }
    onKeyUp(e: KeyboardEvent): Event[] {
        e.preventDefault();

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
        // need to convert into actual coordinates.
        this.icons.renderCursor(context, [this.cursor[0] - this.camera.leftX, this.cursor[1] - this.camera.topY]);
        // can render cursor and popups in this thing 
    }

    update(selectedTile: SelectedTile, entity?: Entity): void { }


}

export class DrawHandler implements InteractionHandler {

    mouseMode: MouseMode;
    clickBounds: SquareCounter;
    selectedTile: SelectedTile;
    camera: Camera;
    cursor: [number, number];
    icons: Icons;


    constructor(selectedTile: SelectedTile, camera: Camera, icons: Icons, cursor: [number, number]) {
        this.mouseMode = new MouseMode();
        this.selectedTile = selectedTile;
        this.camera = camera;
        this.icons = icons;
        this.cursor = cursor;
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
        // should cover popup/tooltip type things when you hover over things 
        // otherwise just the cursor
        this.cursor = xy;
        if (this.mouseMode.get().major === "RANGE") {
            this.clickBounds.lim(xy);
        }
        return [];
    }
    onEnd?(xy: [number, number]): Event[] {
        let mode = this.mouseMode.get();
        if (mode.major === "RANGE") {
            let bounds = this.clickBounds.bounds();
            if (mode.minor === "DRAW") {
                this.mouseMode.reset();
                return new Array({
                    type: EventType.GAME, action: {
                        kind: ActionType.Fill,
                        bounds: bounds,
                        tileset: this.getTileLayer(),
                        idx: this.selectedTile.idx
                    }
                })
                // return a bunch of create tiles events 
            } else if (mode.minor === "CLEAR" || mode.minor === "CLEARALL") {
                this.mouseMode.reset();
                return new Array({
                    type: EventType.GAME, action: {
                        kind: ActionType.Clear,
                        bounds: bounds,
                        layer: 2
                    }
                })
                // return a bunch of clear tile events 
            }
        }
        return [];
    }

    onLeave?(xy: [number, number]): Array<Event> {
        this.mouseMode.reset();
        return [];
    }

    onKeyDown(e: KeyboardEvent): Event[] {
        return [];
    }
    onKeyUp(e: KeyboardEvent): Event[] {
        e.preventDefault();
        switch (e.code) {
            case "Digit1":
                this.mouseMode.reset();
                return new Array({ type: EventType.DISPLAY, action: { kind: UActionType.PopupDungeon } })
            case "Digit2":
                this.mouseMode.reset();
                return new Array({ type: EventType.DISPLAY, action: { kind: UActionType.PopupFeature } })
            case "ArrowDown": // down arrow
                this.camera.down();
                this.mouseMode.reset();
                break;
            case "ArrowUp": // up arrow
                this.camera.up();
                this.mouseMode.reset();
                break;
            case "ArrowLeft": // left arrow
                this.camera.left();
                this.mouseMode.reset();
                break;
            case "ArrowRight": // right arrow
                this.camera.right();
                this.mouseMode.reset();
                break;
            case "Escape":
                this.mouseMode.reset();
                break;
        }
        return [];
    }
    onKeyPressed(e: KeyboardEvent): Event[] {
        if (e.code === "KeyD" && e.shiftKey) {
            this.mouseMode.setMinorClearAll() // need a way to flip back to whatever it was before.
        }
        return [];
    }
    render(context: CanvasRenderingContext2D): void {
        this.icons.renderCursor(context, [this.cursor[0] - this.camera.leftX, this.cursor[1] - this.camera.topY]);
        if (this.mouseMode.get().major === "RANGE") {
            let pattern = context.fillStyle
            context.fillStyle = "blue";
            let bounds = this.clickBounds.bounds();
            context.globalAlpha = 0.25;
            context.fillRect(
                (bounds.x[0] - this.camera.leftX) * 24,
                (bounds.y[0] - this.camera.topY) * 24,
                (bounds.x[1] - this.camera.leftX) * 24 + 24 - (bounds.x[0] - this.camera.leftX) * 24,
                (bounds.y[1] - this.camera.topY) * 24 + 24 - (bounds.y[0] - this.camera.topY) * 24
            );
            context.fillStyle = pattern;
            context.globalAlpha = 1;
        }
    }

    getTileLayer(): number {
        if (this.selectedTile.sheet === "dungeon") {
            return 0;
        } else {
            return 1;
        }
    }
    update(selectedTile: SelectedTile, entity?: Entity): void {
        this.selectedTile = selectedTile;
    }

}

export class MoveHandler implements InteractionHandler {

    camera: Camera;
    cursor: [number, number];
    icons: Icons;
    state: MoveEntityStart

    constructor(camera: Camera, icons: Icons, cursor: [number, number], state: MoveEntityStart) {
        this.camera = camera;
        this.icons = icons;
        this.cursor = cursor;
        this.state = state;
    }

    onClick?(xy: [number, number]): Event[] {
        console.log(`click: ${xy[0]} ${xy[1]}`)
        return new Array({
            type: EventType.GAME,
            action: {
                kind: ActionType.MoveToken,
                entity: this.state.entity,
                from: this.state.xy,
                to: xy
            }
        },
            {
                type: EventType.DISPLAY,
                action: {
                    kind: UActionType.Reset
                }
            }
        );
    }

    onMove?(xy: [number, number]): Event[] {
        this.cursor = xy;
        return [];
    }

    onEnd?(xy: [number, number]): Event[] {
        return [];
    }
    onLeave?(xy: [number, number]): Array<Event> {
        return (new Array({
            type: EventType.DISPLAY,
            action: {
                kind: UActionType.Reset
            }
        }));
    }

    onKeyDown(e: KeyboardEvent): Event[] {
        return [];
    }
    onKeyUp(e: KeyboardEvent): Event[] {
        e.preventDefault();
        switch (e.code) {
            case "Escape":
                return (new Array({
                    type: EventType.DISPLAY,
                    action: {
                        kind: UActionType.Reset
                    }
                }))
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
        this.icons.renderCursor(context, [this.state.xy[0] - this.camera.leftX, this.state.xy[1] - this.camera.topY]);
        this.icons.renderSelectionCursor(context, [this.cursor[0] - this.camera.leftX, this.cursor[1] - this.camera.topY]);
    }

    update(selectedTile: SelectedTile, entity?: Entity): void { }

}

export class PlaceHandler implements InteractionHandler {

    camera: Camera;
    cursor: [number, number];
    icons: Icons;

    constructor(camera: Camera, icons: Icons, cursor: [number, number]) {
        this.camera = camera;
        this.icons = icons;
        this.cursor = cursor;
    }

    onClick?(xy: [number, number]): Event[] {
        return new Array({ type: EventType.DISPLAY, action: { kind: UActionType.PlaceToken, xy: xy } })
    }
    onMove?(xy: [number, number]): Event[] {
        this.cursor = xy;
        return [];
    }
    onEnd?(xy: [number, number]): Event[] {
        return [];
    }
    onLeave?(xy: [number, number]): Array<Event> {
        return [];
    }
    onKeyDown(e: KeyboardEvent): Event[] {
        return [];
    }
    onKeyUp(e: KeyboardEvent): Event[] {
        e.preventDefault();
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
        this.icons.renderCursor(context, [this.cursor[0] - this.camera.leftX, this.cursor[1] - this.camera.topY]);
    }

    update(selectedTile: SelectedTile, entity?: Entity): void { }

}