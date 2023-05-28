<script>
    import { getContext } from "svelte";
    // import { entityStateCb } from "../stores/UI";

    const { close } = getContext("simple-modal");

    export let entities;
    export let entityType;

    // we are going to have pass a callback back to main app.
    // can I use a dispatcher again?

    function onSubmit(e) {
        const formData = new FormData(e.target);

        const data = {};
        for (let field of formData) {
            const [key, value] = field;
            data[key] = value;
        }
        if (data.c !== undefined || data.c !== "") {
            // I want to construct a callback here
            // I don't know where the thing is being spawned yet
            // so I can't add this entity yet
        }
        console.log(data);
        close();
    }
</script>

<!-- // https://www.thisdot.co/blog/handling-forms-in-svelte/ // need to figure out -->
<!-- how to render the characters that are already here -->

<form on:submit|preventDefault={onSubmit}>
    <div>
        <label for="name">Character</label>
        <input type="text" id="character" name="character" value="" />
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
