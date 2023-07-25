<!-- <script>
    import { getContext } from "svelte";
    import EntityForm from "./EntityForm.svelte";

    export let entities;

    const { open } = getContext("simple-modal");

    const showPlayerPicker = () => {
        open(EntityForm, {
            entities: entities,
        });
    };

    const showerOtherPicker = () => {};

    // const showFloorWallPicker = () => {
    //     open(Palette, {
    //         tilesheet: tileSheet.dungeon,
    //         height: 6 * 24,
    //         icons: tileSheet.icon,
    //         store: dungeonTileStore,
    //         sheetName: "dungeon",
    //     });
    // };

    // const showFeaturePicker = () => {
    //     open(Palette, {
    //         tilesheet: tileSheet.feature,
    //         height: 2 * 24,
    //         icons: tileSheet.icon,
    //         store: featureTileStore,
    //         sheetName: "feature",
    //     });
    // };
</script>

<section>
    <button on:click={showerOtherPicker}>Add NPCs</button>
    <br />
    <button on:click={showPlayerPicker}>Add Players</button>
    <br />
</section>

<style>
    section {
        padding-top: 0.5em;
    }

    /* #state {
		position: absolute;
		top: 0;
		right: 0;
		opacity: 0.33;
		font-size: 0.8em;
	} */
</style> -->
