
export enum LoginStateType {
    Login,
    Load,
    Error,
}

export type Login = {
    kind: LoginStateType.Login
}


export type LoadMap = {
    kind: LoginStateType.Load
    id: string
}

export type Error = {
    kind: LoginStateType.Error
    text: string
}

export type LoginState = Login | LoadMap | Error
