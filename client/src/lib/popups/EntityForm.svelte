<script>
    import { getContext } from "svelte";
    import { onDestroy } from "svelte";
    import { EntityType } from "../domain/EntityRenderer";
    import { entityEventStore } from "../stores/UI";
    import { ActionType, EventType } from "../game/Interaction";
    const { close } = getContext("simple-modal");

    export let entities;
    export let xy;
    export let callback;
    let characterValue = "";
    let tokens = 1;

    function onSubmit(e) {
        const formData = new FormData(e.target);

        const data = {};
        for (let field of formData) {
            const [key, value] = field;
            data[key] = value;
        }

        // todo: move entity ID to a call to the server?
        if (data.character.length === 1 && data.description !== "") {
            let entity = {
                c: data.character,
                type: entityType(data.token),
                id: Math.floor(Date.now() + Math.random() * 2000000).toString(), // move serverside
            };

            // todo: differentiate between entities that already exist and those that don't
            let labelAction = {
                kind: ActionType.TokenDescription,
                side: entityType(data.token),
                token: data.character,
                desc: data.description,
            };

            let entityAction = {
                kind: ActionType.AddToken,
                entity: entity,
                xy: xy,
            };

            let gameEvents = [];
            gameEvents.push({ type: EventType.GAME, action: labelAction });
            gameEvents.push({ type: EventType.GAME, action: entityAction });

            entityEventStore.set(gameEvents);
        }
        close();
    }

    const entityType = (s) => {
        if (s === "0") {
            return EntityType.PLAYER;
        } else {
            return EntityType.NPC;
        }
    };

    onDestroy(() => {
        callback();
    });
</script>

<!-- // https://www.thisdot.co/blog/handling-forms-in-svelte/ // need to figure out -->
<!-- how to render the characters that are already here -->

<form
    on:submit|preventDefault={onSubmit}
    on:close={callback}
    autocomplete="off"
>
    <div>
        <label>
            <input type="radio" bind:group={tokens} name="token" value={0} />
            Player Token
        </label>

        <label>
            <input type="radio" bind:group={tokens} name="token" value={1} />
            NPC Token
        </label>
    </div>
    <div>
        <label for="name">Character</label>
        <input
            type="text"
            id="character"
            name="character"
            bind:value={characterValue}
            on:input={() => (characterValue = characterValue.substring(0, 1))}
            autofocus
        />
    </div>
    <div>
        <label for="name">Description</label>
        <input type="text" id="description" name="description" value="" />
    </div>
    <button type="submit">Submit</button>
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
