import type { Camera } from "../game/Camera";
import { EventType, type Event, type GameEvent } from "../game/Interaction";
import type { MapState } from "./DungeonMap";
import type { EntityState } from "./EntityRenderer";


export interface EventSystem {
    event(e: Event): void
    render(context: CanvasRenderingContext2D): void
}

export type GameContext = {
    gameId: string,
    levelId: string,
    userId: string
}

export class RemoteEventSystem implements EventSystem {
    private underlying: LocalEventSystem
    private socket: any
    private context: CanvasRenderingContext2D;
    private gameId: string
    private levelId: string
    private userId: string

    constructor(eventSystem: LocalEventSystem, store: any, context: CanvasRenderingContext2D, gameContext: GameContext) {
        this.underlying = eventSystem;
        this.socket = store;
        this.context = context;
        this.gameId = gameContext.gameId;
        this.levelId = gameContext.levelId;
        this.userId = gameContext.userId;
        this.socket.subscribe((value) => {
            console.log("received message: " + JSON.stringify(value));
            // note all my events are incorrect need to figure them out
            // the next line throws because we send the wrong kind of events. 
            if (value !== undefined && value !== null && Object.keys(value).length !== 0) {
                if (this.isLocal(value["type"])) {
                    this.event_local({
                        type: EventType.GAME,
                        action: value
                    })
                }
            };
        });
    }

    isLocal(s: string): boolean {
        if (s !== "TextMessage") {
            return true
        } else {
            return false
        }
    }

    // sends the websocket the event
    // when we recieve it back it goes to event_local via the subscribe method 
    event(e: GameEvent): void {
        // need to match here to only send the events I need to into the underlying local system.
        let a = e.action;
        switch (a.type) {
            case "TextMessage":
                console.log("text message" + JSON.stringify(e));
                break;
            default:
                this.socket.set(a);;
        }
    }

    event_local(e: GameEvent): void {
        this.underlying.event(e);
        this.render(this.context);
    }

    render(context: CanvasRenderingContext2D): void {
        this.underlying.render(context)
    }
}

export class LocalEventSystem implements EventSystem {
    private map: MapState
    private entities: EntityState
    private camera: Camera
    private defaultTile: number;

    constructor(map: MapState, entities: EntityState, camera: Camera) {
        this.map = map;
        this.entities = entities;
        this.camera = camera;
    }

    event(e: GameEvent): void {
        let a = e.action;
        console.log("inside local eventystem");
        console.log(a);
        switch (a.type) {
            case "TilePlaced":
                if (a.tileset === 0) {
                    this.map.addDungeon(a.x, a.y, a.idx);
                } else {
                    this.map.addFeature(a.x, a.y, a.idx);
                }
                break;
            case "Fill":
                for (let x = a.bounds.x; x <= a.bounds.xx; x++) {
                    for (let y = a.bounds.y; y <= a.bounds.yy; y++) {
                        if (a.tileset === 0) {
                            this.map.addDungeon(x, y, a.idx);
                        } else {
                            this.map.addFeature(x, y, a.idx);
                        }
                    }
                }
                break;
            case "Clear":
                for (let x = a.bounds.x; x <= a.bounds.xx; x++) {
                    for (let y = a.bounds.y; y <= a.bounds.yy; y++) {
                        if (a.layer === 2) {
                            this.map.removeDungeon(x, y);
                            this.map.removeFeature(x, y);
                        } else if (a.layer === 1) {
                            this.map.removeFeature(x, y);
                        } else {
                            this.map.removeDungeon(x, y);
                        }
                    }
                }
                break;
            // case ActionType.TokenDescription:
            //     this.entities.updateLabel(a.token, a.desc); // todo: doesn't tkae into account side
            //     break;
            case "AddToken":
                this.entities.addEntity(a.entity);
                break;
            case "RemoveToken":
                this.entities.remove(a.entity);
                break;
            case "MoveToken":
                this.entities.move(a.entity, a.to[0], a.to[1]);
                break;
            default:
                console.log("unknown token");
                console.log(a);
                console.log("end unknown token");
        }
    }

    render(context: CanvasRenderingContext2D): void {
        let palette = context.fillStyle;
        for (let x = this.camera.leftX; x <= this.camera.rightX; x++) {
            for (let y = this.camera.topY; y <= this.camera.bottomY; y++) {
                this.map.render(context, x, y);
                this.entities.render(context, x, y);
            }
        }
        context.fillStyle = palette;
    }
}