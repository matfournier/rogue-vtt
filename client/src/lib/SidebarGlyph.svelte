<script>
    import { onDestroy } from "svelte";

    // the name is updated whenever the prop value changes...
    export let glpyh;
    export let description;
    export let xy;

    // observe in the console which entry is removed
    // maybe fire an event to the envit system?
    onDestroy(() => {
        // console.log("thing destroyed: " + name);
    });
</script>

<p>
    {xy[0]},{xy[1]}: {glpyh} - {description}
</p>

<style>
    p {
        margin: 0.8em 0;
        color: aliceblue;
    }
    span {
        display: inline-block;
        padding: 0.2em 1em 0.3em;
        text-align: center;
        border-radius: 0.2em;
        background-color: #ffdfd3;
    }
</style>
