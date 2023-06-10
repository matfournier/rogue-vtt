
export enum EntityType {
    PLAYER,
    NPC
}

export type Entity = {
    c: string
    type: EntityType
    id: string // TODO this should come from the server/.
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

    remove(entity: Entity): Stack {
        this.stack.filter(e => e !== entity)
        this.count = this.stack.length
        if (this.count === 0) {
            return undefined;
        }
        else if (this.count === 1) {
            let entity = this.stack[0]
            this.c = entity.c
            this.stackType = this.stackTypeFromEntityType(entity)
        } else {
            this.c = this.count.toString();
            this.stackType = this.getStackType(this.stack)
        }
        return this;
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

    constructor() {
        this.labels = new Map()
        this.colours = new Map()
        this.state = new EntityRenderer(this.colours, this.labels)
        this.colours.set("0", "blue") // pc 
        this.colours.set("1", "white") // pcs 
        this.colours.set("2", "yellow") // npc 
        this.colours.set("3", "orange") // npcs
        this.colours.set("4", "red") // both
    }

    list(x: number, y: number): Stack | undefined {
        return this.state.map.get(this.state.key(x, y))
    }

    updateLabel(c: string, description: string) {
        this.labels.set(c, description)
    }

    addEntity(entity: Entity, x: number, y: number) {
        this.state.put(entity, x, y)
    }

    removeEntityAt(entity: Entity, x: number, y: number) {
        this.state.remove(entity, x, y)
    }

    removeAll(entity: Entity) {
        this.state.removeAll(entity)
        this.labels.delete(entity.c)
    }

    updateColours(type: string, colour: string) {
        this.colours.set(type, colour)
    }
    // entityDetails(): Array<FullySpecifiedEntitiy> {
    //     let res: Array<FullySpecifiedEntitiy> = new Array()
    //     this.state.map.forEach((stack, coord) =>
    //         stack.stack.forEach((entity) => )
    //     )
    // }
    render(context: CanvasRenderingContext2D) {
        this.state.render(context)
    }
}

export class EntityRenderer {
    map: Map<string, Stack>
    labels: Map<string, string> // "g" -> "goblin"
    colours: Map<string, string> // "npc" -> colour 

    constructor(colours: Map<string, string>, labels: Map<string, string>) {
        this.map = new Map()
        this.labels = labels;
        this.colours = colours;
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

    key(x: number, y: number): string {
        return `${x}-${y}`
    }

    private idx(s: string): [number, number] {
        const split = s.split("-")
        return [parseInt(split[0]), parseInt(split[1])]
    }

    remove(e: Entity, x: number, y: number): void {
        let key = this.key(x, y)
        let stack = this.map.get(key)
        if (stack !== undefined) {
            let removed = stack.remove(e)
            if (removed !== undefined) {
                this.map.set(key, stack.remove(e)) // check if I can just mutate this in place.
            } else {
                this.map.delete(key)
            }
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

    render(context: CanvasRenderingContext2D): void {
        let palette = context.fillStyle

        this.map.forEach((stack, coords) => {
            const xy = this.idx(coords)
            let colour = this.colours.get(stack.stackType.toString())
            context.fillStyle = colour
            context.fillText(stack.c, (xy[0] * 24) + 4, (xy[1] * 24) + 19)
        })

        context.fillStyle = palette

        // context.fillStyle = "grey";
        // context.fillText("A", 40 * 24, 18 * 24);
        // context.fillText("B", 42 * 24, 20 * 24);

        // context.fillStyle = "yellow";

        // context.fillText("g", 44 * 24, 19 * 24);
        // context.fillText("g", 42 * 24, 21 * 24);
        // context.fillText("g", 40 * 24, 21 * 24);
        // context.fillText("g", 42 * 24, 22 * 24);

    }
}