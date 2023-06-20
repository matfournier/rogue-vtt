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
            glpyh={player.entity.c}
            description={player.description}
            xy={player.xy}
            colour={player.colour}
        />
    {/each}

    <!-- <h2>Other</h2> -->
    {#each $entityStore.npcs as npc (npc.entity.id)}
        <SidebarGlyph
            entity={npc.entity}
            glpyh={npc.entity.c}
            description={npc.description}
            xy={npc.xy}
            colour={npc.colour}
        />
    {/each}
</div>

<style>
    div {
        display: inline;
    }
</style>
