<script>
	import { onMount } from "svelte";
	import { Grid } from "../domain/Grid";
	import { selectedTileStore } from "../stores/UI";
	import { getContext } from "svelte";

	const { close } = getContext("simple-modal");

	let tileSize = 24;

	const width = 32 * tileSize;
	export let height;
	export let tilesheet;
	export let icons;
	export let store;
	export let sheetName;

	let canvas;
	let context;

	let t, l;

	let selectedTileXY = [0, 0];
	let selectedTileIdx;

	onMount(() => {
		context = canvas.getContext("2d");
		tilesheet.render(context);
	});

	const handleStart = ({ offsetX: x, offsetY: y }) => {
		selectedTileIdx =
			selectedTileXY[1] * (width / tileSize) + selectedTileXY[0];
		store.set({ sheet: sheetName, idx: selectedTileIdx });
		selectedTileStore.set({ sheet: sheetName, idx: selectedTileIdx });
		close();
	};

	const handleMove = ({ offsetX: x1, offsetY: y1 }) => {
		let maybeNewTile = Grid.getTileCoords(x1, y1);
		// console.log(`handleMove: ${maybeNewTile[0]} ${maybeNewTile[1]}`);
		if (
			selectedTileXY[0] != maybeNewTile[0] ||
			selectedTileXY[1] != maybeNewTile[1]
		) {
			selectedTileXY = maybeNewTile;
			context.clearRect(
				0,
				0,
				context.canvas.width,
				context.canvas.height
			);
			tilesheet.render(context);
			icons.renderCursor(context, selectedTileXY);
		}
	};
</script>

<canvas
	{width}
	{height}
	bind:this={canvas}
	on:mousedown={handleStart}
	on:touchstart={(e) => {
		const { clientX, clientY } = e.touches[0];
		handleStart({
			offsetX: clientX - l,
			offsetY: clientY - t,
		});
	}}
	on:mousemove={handleMove}
	on:touchmove={(e) => {
		const { clientX, clientY } = e.touches[0];
		handleMove({
			offsetX: clientX - l,
			offsetY: clientY - t,
		});
	}}
/>
