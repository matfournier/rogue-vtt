import type { Entity, EntityType } from "../domain/EntityRenderer"
import type { Bounds } from "../domain/Grid"

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
    Reset
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
    onClick?(xy: [number, number]): Array<Event>
    onMove?(xy: [number, number]): Array<Event>
    onEnd?(xy: [number, number]): Array<Event>
    onKeyDown(e: KeyboardEvent): Array<Event>
    onKeyUp(e: KeyboardEvent): Array<Event>
    onKeyPressed(e: KeyboardEvent): Array<Event>
    render(context: CanvasRenderingContext2D): void
}

export class ViewHandler implements InteractionHandler {
    onClick?(xy: [number, number]): Event[] {
        throw new Error("Method not implemented.")
    }
    onMove?(xy: [number, number]): Event[] {
        // should cover popup/tooltip type things when you hover over things 
        throw new Error("Method not implemented.")
    }
    onEnd?(xy: [number, number]): Event[] {
        throw new Error("Method not implemented.")
    }
    onKeyDown(e: KeyboardEvent): Event[] {
        throw new Error("Method not implemented.")
    }
    onKeyUp(e: KeyboardEvent): Event[] {
        console.log(`ViewHandler saw ${e.code}`);
        switch (e.code) {
            case "KeyG":
                console.log("saw keyG");
                break;
        }
        return [];
        throw new Error("Method not implemented.")
    }
    onKeyPressed(e: KeyboardEvent): Event[] {
        throw new Error("Method not implemented.")
    }
    render(context: CanvasRenderingContext2D): void {
        // can render cursor and popups in this thing 

    }

}

export class DrawHandler implements InteractionHandler {
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

}