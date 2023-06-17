import { EventType, type Event, ActionType } from "../game/Interaction";
import type { MapState } from "./DungeonMap";


export interface EventSystem {
    event(e: Event): void
}

export class LocalEventSystem implements EventSystem {
    private map: MapState

    constructor(map: MapState) {
        this.map = map;
    }

    event(e: Event): void {
        switch (e.type) {
            case EventType.GAME:
                let a = e.action
                switch (a.kind) {
                    case ActionType.TilePlaced:
                        if (a.tileset === 0) {
                            this.map.addDungeon(a.xy[0], a.xy[1], a.idx);
                        } else {
                            this.map.addFeature(a.xy[0], a.xy[1], a.idx);
                        }
                        break;
                    case ActionType.Fill:
                        for (let x = a.bounds.x[0]; x < a.bounds.x[1]; x++) {
                            for (let y = a.bounds.y[0]; y < a.bounds.y[1]; y++) {
                                if (a.tileset === 0) {
                                    this.map.addDungeon(x, y, a.idx);
                                } else {
                                    this.map.addFeature(x, y, a.idx)
                                }
                            }
                        }
                        break;
                }
                break;
        }

    }
}