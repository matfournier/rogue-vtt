import type { Camera } from "../game/Camera";
import { EventType, type Event, ActionType, type GameEvent } from "../game/Interaction";
import type { MapState } from "./DungeonMap";
import type { EntityState } from "./EntityRenderer";


export interface EventSystem {
    event(e: Event): void
    render(context: CanvasRenderingContext2D): void
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
        console.log(a);
        switch (a.kind) {
            case ActionType.TilePlaced:
                if (a.tileset === 0) {
                    this.map.addDungeon(a.xy[0], a.xy[1], a.idx);
                } else {
                    this.map.addFeature(a.xy[0], a.xy[1], a.idx);
                }
                break;
            case ActionType.Fill:
                for (let x = a.bounds.x[0]; x <= a.bounds.x[1]; x++) {
                    for (let y = a.bounds.y[0]; y <= a.bounds.y[1]; y++) {
                        if (a.tileset === 0) {
                            this.map.addDungeon(x, y, a.idx);
                        } else {
                            this.map.addFeature(x, y, a.idx);
                        }
                    }
                }
                break;
            case ActionType.Clear:
                for (let x = a.bounds.x[0]; x <= a.bounds.x[1]; x++) {
                    for (let y = a.bounds.y[0]; y <= a.bounds.y[1]; y++) {
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
            case ActionType.AddToken:
                this.entities.addEntity(a.entity);
                break;
            case ActionType.RemoveToken:
                this.entities.remove(a.entity);
                break;
            case ActionType.MoveToken:
                console.log(a)
                this.entities.move(a.entity, a.to[0], a.to[1]);
                break;
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