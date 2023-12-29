import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/function";
import type { Either } from "fp-ts/Either";
import { LoginActionType } from "../login/loginstate";
import type { LoadGame, LoginAction } from "../login/loginstate"
import { LevelKind } from "../transfer/Transfer";

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

export function parseLogin(form: any): LoginAction {
    console.log(form);
    return pipe(
        formToObject(form),
        E.map(a => parseLoginForm(a)),
        E.fold(e => { return <LoginAction>{ kind: LoginActionType.Error, text: e } }, r => r)
    )

}

function parseLoginForm(e: any): LoginAction {
    const getKey = (v: string): Either<string, any> => {
        const result = e[v];
        if (result == null || result == undefined) {
            return E.left(`Could not find ${v} on form`)
        } else {
            return E.right(result)
        }
    }


    const parseType = (v: string): Either<string, LoginActionType> => {
        if (v == "0") {
            return E.right(LoginActionType.Create)
        } else if (v == "1") {
            return E.right(LoginActionType.Load)
        } else {
            return E.left("Could not parse form")
        }
    }

    const validNumber = (v: number): Either<string, number> => {
        if (v >= 0 && v <= 1000) {
            return E.right(v)
        } else {
            return E.left(`${v} is not between 0 and 1000`)
        }
    }

    const parseXY = (): Either<string, [number, number]> => {
        return pipe(
            pipe(getKey("X"), E.chain(n => validNumber(n as number)),
                E.chain(x => pipe(getKey("Y"), E.chain(n => validNumber(n as number)), E.map(y => [x, y])))
            )
        )
    }

    const nonEmpty = (b: string): Either<string, string> => {
        if (b == null || b == undefined || b == "") {
            return E.left("Loading a map from an empty ID is not possible")
        } else {
            return E.right(b)
        }
    }

    const parseMapId = (): Either<string, LoadGame> =>
        pipe(
            getKey("ID"),
            E.chain(nonEmpty),
            E.map(<LoadGame>(id: string) => { return { kind: LoginActionType.Load, id: id } })
        )

    const parseLoginType = (t: LoginActionType): Either<string, LoginAction> => {
        if (t == LoginActionType.Create) {
            return pipe(
                parseXY(),
                E.map(xy => { return { kind: LoginActionType.Create, xy: xy, description: "default", user: "default", levelKind: LevelKind.Dungeon } })
            )
        } else {
            return parseMapId()
        }
    }


    return pipe(
        getKey("newmap"),
        E.chain(parseType),
        E.chain(parseLoginType),
        E.fold(e => { return { kind: LoginActionType.Error, text: e } }, r => r)
    )
}