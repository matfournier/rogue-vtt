<script>
    import { getContext } from "svelte";
    import { parseLogin } from "../tool/FormParser";
    import { LoginActionType, LoginResultType } from "./loginstate";
    import { parseGamestate } from "../transfer/Transfer";
    import { loginStateStore } from "../stores/UI";

    let characterValue = "";
    let radio = 0;
    let error = false;
    const host = import.meta.env.VITE_ROGUE_HOST_NAME;
    const httpType = import.meta.env.VITE_HTTP_TYPE;

    async function onSubmit(e) {
        const f = parseLogin(e);
        console.log(f);
        // todo make calls to backend here and make this fn async
        if (f.kind == LoginActionType.Create) {
            let url = `${httpType}://${host}/create_game?mode=${f.levelKind}&x=${f.xy[0]}&y=${f.xy[1]}&description=${f.description}&user=${f.user}`;
            fetch(url, { method: "POST" })
                .then((response) => {
                    console.log(`response status: ${response.status}`);
                    if (response.status == 200) {
                        return response.json(); // we get a string back
                    } else {
                        return new Promise((resolve, _) => {
                            response.text().then((t) => {
                                console.log(t);
                                resolve();
                            });
                        });
                    }
                })
                .then((result) => {
                    console.log(result); // string can be empty here
                    let gs = parseGamestate(result);
                    loginStateStore.set({
                        kind: LoginResultType.Load,
                        game: gs,
                    });
                });
        } else if (f.kind == LoginActionType.Load) {
            let url = `${httpType}://${host}/load_game/${f.id}`;
            fetch(url)
                .then((response) => {
                    console.log(`response status: ${response.status}`);
                    if (response.status == 200) {
                        return response.json();
                    } else {
                        return new Promise((resolve, _) => {
                            response.text().then((t) => {
                                console.log(t);
                                resolve();
                            });
                        });
                    }
                })
                .then((result) => {
                    console.log(result); // string can be empty here
                    let gs = parseGamestate(result);
                    loginStateStore.set({
                        kind: LoginResultType.Load,
                        game: gs,
                    });
                });
        } else {
            loginStateStore.set({
                kind: LoginResultType.Error,
                error: f.text,
            });
        }
    }
</script>

<form on:submit|preventDefault={onSubmit} autocomplete="off">
    <div>
        <label>
            <input type="radio" bind:group={radio} name="newmap" value={0} />
            New Map
        </label>
        <label>
            <input type="radio" bind:group={radio} name="newmap" value={1} />
            Existing Map
        </label>
    </div>
    {#if radio == 1}
        <div>
            <label for="name">ID</label>
            <input type="text" id="ID" name="ID" value="" />
        </div>
    {:else}
        <div>
            <label for="x">x</label>
            <input type="number" id="X" name="X" value="250" />

            <label for="x">y</label>
            <input type="number" id="Y" name="Y" value="250" />
        </div>
    {/if}
    <button type="submit">Play</button>
</form>
<section />

<style>
    * {
        box-sizing: border-box;
    }
    form {
        display: flex;
        flex-direction: column;
        width: 300px;
        color: white;
    }

    form > div {
        display: flex;
        justify-content: space-between;
    }

    form > div + * {
        margin-top: 10px;
    }
</style>
