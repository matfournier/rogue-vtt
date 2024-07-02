<script>
    import SidebarGlyph from "./SidebarGlyph.svelte";
    import { entityStore } from "./stores/UI";
    // export let entityState;
</script>

<div>
    <!-- <h2>Players</h2> -->
    {#each $entityStore.players as player (player.entity.id)}
        <SidebarGlyph
            entity={player.entity}
            glpyh={player.entity.character}
            description={player.entity.description}
            xy={[player.entity.x, player.entity.y]}
            colour={player.colour}
        />
    {/each}

    <!-- <h2>Other</h2> -->
    {#each $entityStore.npcs as npc (npc.entity.id)}
        <SidebarGlyph
            entity={npc.entity}
            glpyh={npc.entity.character}
            description={npc.entity.description}
            xy={[npc.entity.x, npc.entity.y]}
            colour={npc.colour}
        />
    {/each}
</div>

<style>
    div {
        display: inline;
    }
</style>
