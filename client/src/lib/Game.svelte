<script>
	import { onDestroy } from "svelte";
	import { onMount } from "svelte";
	import { Grid } from "./domain/Grid";
	import Toolbar from "./Toolbar.svelte";
	import { MapState } from "./domain/DungeonMap";
	import TilePicker from "./tilepicker/TilePicker.svelte";
	import Modal from "./Modal.svelte";
	import {
		dungeonTileStore,
		featureTileStore,
		selectedTileStore,
	} from "./stores/UI";

	// https://svelte.dev/repl/434e0b14546747688401e8808c060a23?version=3.47.0

	let tIdx = 97;
	$: tileIndex = tIdx;

	const tileSize = 24;
	let dimensions = [50, 25];

	export let width = dimensions[0] * tileSize;
	export let height = dimensions[1] * tileSize;
	export let tileSheet;

	export let background = "#fff";

	// turn all this palette stuff into a class

	let paletteDungeon;
	let paletteFeature;
	let paletteSelected;

	const stores = {
		dungeon: dungeonTileStore.subscribe((value) => {
			paletteDungeon = value;
		}),
		feature: featureTileStore.subscribe((value) => {
			paletteFeature = value;
		}),
		selected: selectedTileStore.subscribe((value) => {
			paletteSelected = value;
		}),
	};

	const defaultCanvas = (tile) => {
		const tiledBackgroundCanvas = document.createElement("canvas");
		const backgroundContext = tiledBackgroundCanvas.getContext("2d");
		tiledBackgroundCanvas.width = tileSize;
		tiledBackgroundCanvas.height = tileSize;

		console.log(`${tile.sx} ${tile.sy} <- default`);
		backgroundContext.drawImage(
			tileSheet.dungeon.src,
			tile.sx,
			tile.sy,
			tileSize,
			tileSize,
			0,
			0,
			tileSize,
			tileSize
		);
		context.save;
		return tiledBackgroundCanvas;
	};

	let canvas;
	let context;
	let selectedMapTile;
	let map;
	let t, l;

	onMount(() => {
		selectedMapTile = [0, 0];

		map = new MapState(dimensions[0], dimensions[1], tileSheet);
		let defaultTile = tileSheet.dungeon.tiles[101];
		context = canvas.getContext("2d");
		let pattern = context.createPattern(
			defaultCanvas(defaultTile),
			"repeat"
		);
		context.fillStyle = pattern;
		handleSize();
		drawTile();
	});

	const teardown = () => {
		stores.entries.forEach((unsub) => unsub());
	};

	onDestroy(teardown);

	function onKeyDown(e) {
		console.log(e.key);
		switch (e.keyCode) {
			case 88: // x
				console.log("x!");
				// deletes our tiles from our tile map
				// for (let idx in tiles) {
				// 	delete tiles[idx];
				// }
				// this clears everything on a canvas
				context.clearRect(
					0,
					0,
					context.canvas.width,
					context.canvas.height
				);
				break;
		}
	}

	const handleStart = ({ offsetX: x, offsetY: y }) => {
		// let [xx, yy] = Logic.getTileCoords(x, y);

		addTileFromPalette(selectedMapTile);
		drawTile();
	};

	const addTileFromPalette = (xy) => {
		if (paletteSelected.sheet === "dungeon") {
			console.log(
				`dungeon click x: ${selectedMapTile[0]}, y: ${selectedMapTile[1]}, tile: ${paletteSelected.idx}`
			);
			map.addDungeon(xy[0], xy[1], paletteSelected.idx);
		} else if (paletteSelected.sheet === "feature") {
			console.log(
				`feature click x: ${selectedMapTile[0]}, y: ${selectedMapTile[1]}, tile: ${paletteSelected.idx}`
			);
			map.addFeature(xy[0], xy[1], paletteSelected.idx);
		} else {
			console.log("ERROR: CANNOT ADD TILE TO MAP");
		}
	};

	const drawTile = () => {
		context.fillRect(0, 0, context.canvas.width, context.canvas.height);
		map.render(context);
		tileSheet.icon.renderCursor(context, selectedMapTile);
	};

	const handleEnd = () => {};

	const handleMove = ({ offsetX: x1, offsetY: y1 }) => {
		let maybeNewTile = Grid.getTileCoords(x1, y1);
		// console.log(`handleMove: ${maybeNewTile[0]} ${maybeNewTile[1]}`);
		if (
			selectedMapTile[0] != maybeNewTile[0] ||
			selectedMapTile[1] != maybeNewTile[1]
		) {
			selectedMapTile = maybeNewTile;
			drawTile();
		}
	};

	const handleSize = () => {
		const { top, left } = canvas.getBoundingClientRect();
		t = top;
		l = left;
	};
</script>

<!-- need to put this on a div wrapping the canvas somehow -->
<!-- on:keydown|preventDefault={onKeyDown}  -->

<svelte:window on:resize={handleSize} on:keydown={onKeyDown} />

<!-- https://dev.to/jdgamble555/the-unwritten-svelte-stores-guide-47la -->
<!-- https://svelte-recipes.netlify.app/components/ -->

<div on:keydown={onKeyDown}>
	<canvas
		{width}
		{height}
		style:background
		bind:this={canvas}
		on:mousedown={handleStart}
		on:touchstart={(e) => {
			const { clientX, clientY } = e.touches[0];
			handleStart({
				offsetX: clientX - l,
				offsetY: clientY - t,
			});
		}}
		on:mouseup={handleEnd}
		on:touchend={handleEnd}
		on:mouseleave={handleEnd}
		on:mousemove={handleMove}
		on:touchmove={(e) => {
			const { clientX, clientY } = e.touches[0];
			handleMove({
				offsetX: clientX - l,
				offsetY: clientY - t,
			});
		}}
	/>
</div>
<Toolbar bind:tileIdx={tileIndex} />
<Modal>
	<TilePicker {tileSheet} />
</Modal>
