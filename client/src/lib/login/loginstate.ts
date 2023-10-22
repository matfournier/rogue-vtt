import type { GameState, LevelKind } from "../transfer/Transfer"

export enum LoginActionType {
    Create,
    Load,
    Error,
}

export type Create = {
    kind: LoginActionType.Create
    levelKind: LevelKind,
    xy: [number, number],
    description: string,
    user: string
}


export type LoadGame = {
    kind: LoginActionType.Load,
    id: string
}

export type LoginError = {
    kind: LoginActionType.Error,
    text: string
}

export type LoginAction = Create | LoadGame | LoginError

export enum LoginResultType {
    NoResult,
    Load,
    Error
}

export type LoadGameResult = {
    kind: LoginResultType.Load,
    game: GameState
}

export type ErrorResult = {
    kind: LoginResultType.Error
    error: string
}

export type NoResult = {
    kind: LoginResultType.NoResult

}
export type LoginResult = NoResult | LoadGameResult | ErrorResult 
