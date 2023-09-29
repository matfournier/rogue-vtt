<script>
    import { getContext } from "svelte";
    import { parseLogin } from "../tool/FormParser";

    let characterValue = "";
    let radio = 0;

    function onSubmit(e) {
        const f = parseLogin(e);
        console.log(f);
        // todo make calls to backend here and make this fn async
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
    {#if radio == "1"}
        <div>
            <label for="name">ID</label>
            <input type="text" id="ID" name="ID" value="" />
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
