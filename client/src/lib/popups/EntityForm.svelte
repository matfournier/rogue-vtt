<script>
    import { getContext } from "svelte";
    import { onDestroy } from "svelte";
    const { close } = getContext("simple-modal");

    export let entities;
    export let entityType;
    export let xy;
    export let callback;

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
                type: entityType,
                id: Math.floor(Date.now() + Math.random() * 2000000).toString,
            };
            entities.updateLabel(data.character, data.description);
            entities.addEntity(entity, xy[0], xy[1]);
        }
        close();
    }

    const bar = (e) => {
        if (e === 0) {
            return "PLAYER";
        } else {
            return "NPC";
        }
    };

    const title = bar(entityType);

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
    <h2>{title}</h2>
    <div>
        <label for="name">Character</label>
        <input type="text" id="character" name="character" value="" autofocus />
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
