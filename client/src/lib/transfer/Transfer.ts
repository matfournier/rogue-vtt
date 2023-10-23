import type { Entity } from "../domain/EntityRenderer"

export type Tile = {
    x: number,
    y: number,
    idx: number
}
export enum LevelKind {
    Dungeon,
    Overland
}

export type Level = {
    kind: LevelKind,
    description: String,
    id: String,
    dimension: [number, number] // cols, rows 
    tiles: Array<Tile>,
    features: Array<Tile>
}


export function entityToJson(e: Entity): String {
    return JSON.stringify(e)
}

// ServerSide this is a Map<EntityId, Entity>
export type Entities = {
    players: Array<Entity>
    npcs: Array<Entity>
}

export type EntitiesMap = {
    players: Map<String, Entity>
    npcs: Map<String, Entity>
}

export function entitiesToJson(e: Entities): EntitiesMap {
    let players: Map<String, Entity> = new Map<String, Entity>();
    let npcs: Map<String, Entity> = new Map<String, Entity>();
    e.players.forEach(e => players.set(e.id, e))
    e.npcs.forEach(e => npcs.set(e.id, e))
    let em: EntitiesMap = {
        players: players,
        npcs: npcs
    }
    return em
}

export type GameState = {
    level: Level,
    entities: Entities
    id: String
}

export function gameStateToJson(gs: GameState): String {
    return JSON.stringify(gs)
}

export function parseGamestate(s: any): GameState {
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
        entities: entities,
        id: s["id"]
    }

}