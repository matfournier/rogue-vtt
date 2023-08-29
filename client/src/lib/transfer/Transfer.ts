
export type Tile = {
    x: number,
    y: number,
    idx: number
}


export type Level = {
    description: String,
    id: String,
    dimension: [number, number] // cols, rows 
    tiles: Array<Tile>,
    features: Array<Tile>
}

export type Entity = {
    kind: String,
    x: number,
    y: number,
    character: String,
    id: String,
    description: String
}

export type Entities = {
    players: Array<Entity>
    npcs: Array<Entity>
}

export type GameState = {
    level: Level,
    entities: Entities
}

export function parse(s: any): GameState {
    // let entities: Entities = s["entities"];
    let e = s["entities"];

    let players = new Array<Entity>();
    Object.keys(e["players"]).forEach((k) => {
        players.push(e["players"][k])
    });

    let npcs = new Array<Entity>();
    Object.keys(e["npcs"]).forEach((k) => {
        players.push(e["npcs"][k])
    });

    let entities: Entities = {
        players: players,
        npcs: npcs
    };


    let level: Level = s["level"];

    return {
        level: level,
        entities: entities
    }

}