<script>
    import SidebarGlyph from "./SidebarGlyph.svelte";
    import { entityStore } from "./stores/UI";
    // export let entityState;

    let npcs = $entityStore.npcs;
    let players = $entityStore.players;
</script>

<div>
    {#each npcs as npc (npc.entity.id)}
        <SidebarGlyph
            glpyh={npc.entity.c}
            description={npc.description}
            xy={npc.xy}
        />
    {/each}
</div>
<div>
    {#each players as player (player.entity.id)}
        <SidebarGlyph
            glpyh={player.entity.c}
            description={player.description}
            xy={player.xy}
        />
    {/each}
</div>
