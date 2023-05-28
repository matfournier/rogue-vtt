<script>
	import { getContext } from "svelte";
	import Palette from "./Palette.svelte";

	export let tileSheet;

	const { open } = getContext("simple-modal");

	const showFloorWallPicker = () => {
		open(Palette, {
			tilesheet: tileSheet.dungeon,
			height: 6 * 24,
			icons: tileSheet.icon,
			sheetName: "dungeon",
		});
	};

	const showFeaturePicker = () => {
		open(Palette, {
			tilesheet: tileSheet.feature,
			height: 2 * 24,
			icons: tileSheet.icon,
			sheetName: "feature",
		});
	};
</script>

<section>
	<button on:click={showFloorWallPicker}>Floors/Walls</button>
	<br />
	<button on:click={showFeaturePicker}>Features</button>
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
</style>
