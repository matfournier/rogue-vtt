<script>
    import { onDestroy } from "svelte";
    import { onMount } from "svelte";
    import { entityEventStore } from "./stores/UI";
    import { ActionType, EventType, UActionType } from "./game/Interaction";

    // the name is updated whenever the prop value changes...
    export let entity;
    export let glpyh;
    export let description;
    export let xy;
    export let colour;

    // observe in the console which entry is removed
    // maybe fire an event to the envit system?

    onDestroy(() => {
        // console.log("thing destroyed: " + name);
    });

    const remove = () => {
        entityEventStore.set([
            {
                type: EventType.GAME,
                action: {
                    kind: ActionType.RemoveToken,
                    entity: entity,
                    xy: xy,
                },
            },
        ]);
    };

    const move = () => {
        console.log("moving");
        entityEventStore.set([
            {
                type: EventType.DISPLAY,
                action: {
                    kind: UActionType.MoveEntityStart,
                    entity: entity,
                    xy: xy,
                },
            },
        ]);
    };
</script>

<p style="color: {colour};" on:click={move}>
    {xy[0]},{xy[1]}: {glpyh} - {description}
    <button on:click={remove}>x</button>
</p>

<style>
    p {
        margin: 0.8em 0;
    }
</style>
