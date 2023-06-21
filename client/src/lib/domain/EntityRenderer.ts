import type { Camera } from "../game/Camera";
import { entityStore } from "../stores/UI";

export enum EntityType {
    PLAYER,
    NPC
}

export type Entity = {
    c: string
    type: EntityType
    id: string // TODO this should come from the server/.
}

export type RichEntity = {
    entity: Entity
    xy: [number, number]
    description: string
    colour: string
}

export type Sidebar = {
    players: Array<RichEntity>
    npcs: Array<RichEntity>
}

export function alphabet(): Array<string> {
    const alphabet = [...'abcdefghijklmnopqrstuvwxyz'];
    const upper = alphabet.map(s => s.toUpperCase())
    return upper.concat(alphabet)
}

enum StackType {
    PLAYER,
    PLAYERS,
    NPC,
    NPCS,
    BOTH
}

class Stack {
    c: string
    stack: Array<Entity>
    count: number
    stackType: StackType

    constructor(entity: Entity) {
        this.stack = new Array(entity);
        this.count = 1;
        this.c = entity.c;
        this.stackType = this.stackTypeFromEntityType(entity)
    }

    add(entity: Entity): Stack {
        this.stack.push(entity)
        this.count = this.stack.length
        if (this.count === 1) {
            this.c = entity.c
            this.stackType = this.stackTypeFromEntityType(entity)
        } else {
            this.c = this.count.toString();
            this.stackType = this.getStackType(this.stack)
        }
        return this;
    }

    remove?(entity: Entity): Stack {
        let newStack = this.stack.filter(e => e.id !== entity.id)
        let count = newStack.length
        if (count === 0) {
            return undefined;
        }
        else if (count === 1) {
            return new Stack(newStack[0]);
        } else {
            this.c = count.toString();
            this.count = count;
            this.stackType = this.getStackType(newStack)
            this.stack = newStack;
            return this;
        }
    }

    private stackTypeFromEntityType(entity: Entity): StackType {
        return (entity.type === EntityType.PLAYER ? StackType.PLAYER : StackType.NPC)
    }

    private getStackType(stack: Array<Entity>): StackType {
        if (stack.length === 1) {
            return this.stackTypeFromEntityType(stack[0])
        } else {
            let type: Set<EntityType> = new Set(stack.map(e => e.type))
            let l = type.size
            if (l === 1 && type.has(EntityType.PLAYER)) {
                return StackType.PLAYERS
            } else if (l === 1 && type.has(EntityType.NPC)) {
                return StackType.NPCS
            } else {
                return StackType.BOTH
            }
        }
    }
}

// export type FullySpecifiedEntitiy {
//     entity: Entity
//     position: [number, number]
//     label: string
// }

export class EntityState {
    private state: EntityRenderer
    labels: Map<string, string>
    colours: Map<string, string>

    constructor(camera: Camera) {
        this.labels = new Map()
        this.colours = new Map()
        this.state = new EntityRenderer(this.colours, this.labels, camera)
        this.colours.set("0", "#348feb") // pc 
        this.colours.set("1", "white") // pcs 
        this.colours.set("2", "yellow") // npc 
        this.colours.set("3", "orange") // npcs
        this.colours.set("4", "red") // both
    }

    list(x: number, y: number): Stack | undefined {
        return this.state.map.get(this.state.key(x, y));
    }

    updateLabel(c: string, description: string) {
        this.labels.set(c, description);
        this.updateEntityStore();

    }

    addEntity(entity: Entity, x: number, y: number) {
        this.state.put(entity, x, y)
        this.updateEntityStore();
    }

    removeEntityAt(entity: Entity, x: number, y: number) {
        this.state.remove(entity, x, y);
        this.updateEntityStore();
        // todo: remove label if it no longer exists 
    }

    removeAll(entity: Entity) {
        this.state.removeAll(entity)
        this.labels.delete(entity.c)
        this.updateEntityStore();
    }

    updateColours(type: string, colour: string) {
        this.colours.set(type, colour);
        this.updateEntityStore();
    }

    render(context: CanvasRenderingContext2D, x: number, y: number) {
        this.state.render(context, x, y)
    }
    move(entity: Entity, x: number, y: number, xx: number, yy: number) {
        this.state.move(entity, x, y, xx, yy);
        this.updateEntityStore();
    }

    updateEntityStore(): void {
        let players: Array<RichEntity> = new Array();
        let npcs: Array<RichEntity> = new Array();
        this.state.map.forEach((stack, coord) => {
            let xy = this.idx(coord);
            stack.stack.forEach(entity => {
                let description = this.labels.get(entity.c)
                if (entity.type === EntityType.PLAYER) {
                    let colour = this.colours.get("0");
                    players.push({ entity: entity, xy: xy, description: description, colour: colour });
                } else {
                    let colour = this.colours.get("2");
                    npcs.push({ entity: entity, xy: xy, description: description, colour: colour });
                }
            })
        }
        )
        players.sort((a, b) => {
            if (a.xy == b.xy) {
                return 0
            } else if (a.xy[0] < b.xy[0]) {
                return -1;
            } else {
                return 1;
            }
        })

        npcs.sort((a, b) => {
            if (a.xy == b.xy) {
                return 0
            } else if (a.xy[0] < b.xy[0]) {
                return -1;
            } else {
                return 1;
            }
        })

        entityStore.set({ players: players, npcs: npcs });
    }

    private idx(s: string): [number, number] {
        const split = s.split("-")
        return [parseInt(split[0]), parseInt(split[1])]
    }
}

export class EntityRenderer {
    map: Map<string, Stack>
    labels: Map<string, string> // "g" -> "goblin" TODO need a different one for PCs and NPCs
    colours: Map<string, string> // "npc" -> colour 
    camera: Camera

    constructor(colours: Map<string, string>, labels: Map<string, string>, camera: Camera) {
        this.map = new Map()
        this.labels = labels;
        this.colours = colours;
        this.camera = camera;
    }

    put(e: Entity, x: number, y: number): void {
        let key = this.key(x, y)
        let stack = this.map.get(key)
        if (stack !== undefined) {
            this.map.set(key, stack.add(e))
        } else {
            let stack = new Stack(e)
            this.map.set(key, stack)
        }

    }

    move(e: Entity, x: number, y: number, xx: number, yy: number): void {
        if (this.remove(e, x, y)) {
            this.put(e, xx, yy);
        }
    }

    key(x: number, y: number): string {
        return `${x}-${y}`
    }

    remove(e: Entity, x: number, y: number): boolean {
        let key = this.key(x, y)
        let stack = this.map.get(key)
        if (stack !== undefined) {
            let removed = stack.remove(e)
            if (removed !== undefined) {
                this.map.set(key, removed)
            } else {
                this.map.delete(key)
            }
            return true;
        } else {
            return false;
        }
    }

    removeAll(e: Entity): void {
        let res: Map<string, Stack> = new Map()
        this.map.forEach((stack, xy) => {
            let newStack = stack.remove(e)
            if (newStack !== undefined) {
                res.set(xy, newStack)
            }
        })
    }

    // render(context: CanvasRenderingContext2D): void {
    //     let palette = context.fillStyle

    //     this.map.forEach((stack, coords) => {
    //         const xy = this.idx(coords)
    //         let colour = this.colours.get(stack.stackType.toString())
    //         context.fillStyle = colour
    //         context.fillText(stack.c, (xy[0] * 24) + 4, (xy[1] * 24) + 19)
    //     })

    //     context.fillStyle = palette

    // }

    render(context: CanvasRenderingContext2D, x: number, y: number): void {
        // let palette = context.fillStyle
        let stack = this.map.get(this.key(x, y));
        if (stack !== undefined) {
            let colour = this.colours.get(stack.stackType.toString())
            context.fillStyle = colour
            context.fillText(stack.c, ((x - this.camera.leftX) * 24) + 4, ((y - this.camera.topY) * 24) + 19)
        }

        // context.fillStyle = palette
    }

}