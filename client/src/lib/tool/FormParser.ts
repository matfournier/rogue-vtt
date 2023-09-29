import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/Function";
import type { Either } from "fp-ts/Either";
import { LoginStateType } from "../login/loginstate";
import type { LoadMap, LoginState } from "../login/loginstate"

// typing a form in typescript appears to be the ninth circle of hell, wtf

export function formToObject(e: any): Either<string, any> {
    try {
        const formData = new FormData(e.target);
        const data: any = {};
        for (let field of formData) {
            const [key, value] = field;
            data[key] = value;
        }

        console.log(data);
        return E.right(data);
    } catch (e: any) {
        return E.left(e.message as string)
    }
}

export function parseLogin(form: any): LoginState {
    console.log(form);
    return pipe(
        formToObject(form),
        E.map(a => parseLoginForm(a)),
        E.fold(e => { return <LoginState>{ kind: LoginStateType.Error, text: e } }, r => r)
    )

}

function parseLoginForm(e: any): LoginState {
    const getKey = (v: string): Either<string, any> => {
        const result = e[v];
        if (result == null || result == undefined) {
            return E.left(`Could not find ${v} on form`)
        } else {
            return E.right(result)
        }
    }


    const parseType = (v: string): Either<string, LoginStateType> => {
        if (v == "0") {
            return E.right(LoginStateType.Login)
        } else if (v == "1") {
            return E.right(LoginStateType.Load)
        } else {
            return E.left("Could not parse form")
        }
    }


    const nonEmpty = (b: string): Either<string, string> => {
        if (b == null || b == undefined || b == "") {
            return E.left("Loading a map from an empty ID is not possible")
        } else {
            return E.right(b)
        }
    }

    const parseMapId = (): Either<string, LoadMap> =>
        pipe(
            getKey("ID"),
            E.chain(nonEmpty),
            E.map(<LoadMap>(id: string) => { return { kind: LoginStateType.Load, id: id } })
        )

    const parseLoginType = (t: LoginStateType): Either<string, LoginState> => {
        if (t == LoginStateType.Login) {
            return E.right({ kind: LoginStateType.Login })
        } else {
            return parseMapId()
        }
    }


    return pipe(
        getKey("newmap"),
        E.chain(parseType),
        E.chain(parseLoginType),
        E.fold(e => { return { kind: LoginStateType.Error, text: e } }, r => r)
    )
}