<script>
	import { onDestroy } from "svelte";
	import { onMount } from "svelte";
	import { Grid, SquareCounter } from "../domain/Grid";
	import Toolbar from "../Toolbar.svelte";
	import { MapState } from "../domain/DungeonMap";
	import TilePicker from "../tilepicker/TilePicker.svelte";
	import Modal from "../Modal.svelte";
	import {
		dungeonTileStore,
		featureTileStore,
		selectedTileStore,
	} from "../stores/UI";
	import { MouseMode } from "./MouseMode";
	import { xlink_attr } from "svelte/internal";

	// https://svelte.dev/repl/434e0b14546747688401e8808c060a23?version=3.47.0

	let tIdx = 97;
	$: tileIndex = tIdx;

	const tileSize = 24;
	let dimensions = [50, 25];

	export let width = dimensions[0] * tileSize;
	export let height = dimensions[1] * tileSize;
	export let tileSheet;

	export let background = "#fff";

	let pattern;

	// turn all this palette stuff into a class

	let paletteDungeon;
	let paletteFeature;
	let paletteSelected;

	const mouseMode = new MouseMode();
	let clickBounds; // when someone clicks, decide if you should try a single or multiple tile

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

	let canvas;
	let context;
	let selectedMapTile;
	let map;
	let t, l;

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

	onMount(() => {
		selectedMapTile = [0, 0];
		map = new MapState(dimensions[0], dimensions[1], tileSheet);
		let defaultTile = tileSheet.dungeon.tiles[101];
		context = canvas.getContext("2d");
		pattern = context.createPattern(defaultCanvas(defaultTile), "repeat");
		context.fillStyle = pattern;
		handleSize();
		draw();
	});

	const teardown = () => {
		stores.entries.forEach((unsub) => unsub());
	};

	onDestroy(teardown);

	function onKeyDown(e) {
		console.log(e.key);
		switch (e.keyCode) {
			case 27: // esc
				modeReset();
				draw();
				break;
		}
	}

	function onKeyUp(e) {
		switch (e.keyCode) {
			case 88: // esc
				mouseMode.reset();
				break;
		}
	}

	function onKeyHeld(e) {
		switch (e.keyCode) {
			// could you implement defining a region and then hitting "delete" to delete it if in no mode?
			// if you are in mode NONE I think this would work the best?
			// this is hacky for now.

			case 88: // x
				if (e.shiftKey) {
					mouseMode.setMinorClear();
				} else {
					mouseMode.setMinorClearAll();
				}
				break;
		}
	}

	const handleStart = () => {
		clickBounds = new SquareCounter(selectedMapTile);
		if (mouseMode.get().major === "NONE") {
			mouseMode.setSelection();
		}
	};

	const modeReset = () => {
		mouseMode.reset();
		clickBounds = undefined;
	};

	// refactor these two things, should take the mode + xy and delegate to a class
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

	// refactor these two things, should take the mode + xy and delegate to a class
	const removeTileFromPalette = (xy) => {
		if (mouseMode.get().minor === "CLEARALL") {
			map.removeDungeon(xy[0], xy[1]);
			map.removeFeature(xy[0], xy[1], paletteSelected.idx);
		} else {
			if (paletteSelected.sheet === "dungeon") {
				map.removeDungeon(xy[0], xy[1]);
			} else if (paletteSelected.sheet === "feature") {
				map.removeFeature(xy[0], xy[1], paletteSelected.idx);
			} else {
				console.log("ERROR: CANNOT ADD TILE TO MAP");
			}
		}
	};

	const draw = () => {
		context.fillRect(0, 0, context.canvas.width, context.canvas.height);
		map.render(context);

		tileSheet.icon.renderCursor(context, selectedMapTile);

		context.font = "18pt Monospace";
		context.fillStyle = "grey";
		context.fillText("1", 6, 18);
		context.fillStyle = pattern;
	};

	// mouse up && when someone leaves the screen, need to think about this.
	const handleEnd = () => {
		// console.log(tiles);
		let mode = mouseMode.get();
		if (mode.major === "SELECTION") {
			let tiles = clickBounds.tiles();
			if (mode.minor === "DRAW") {
				tiles.forEach((tile) => addTileFromPalette(tile));
			} else if (mode.minor === "CLEAR" || mode.minor === "CLEARALL") {
				tiles.forEach((tile) => removeTileFromPalette(tile));
			}
		}
		modeReset();
		draw();
	};

	const handleExit = () => {
		modeReset();
		draw();
	};

	const handleMove = ({ offsetX: x1, offsetY: y1 }) => {
		let maybeNewTile = Grid.getTileCoords(x1, y1);
		// console.log(`handleMove: ${maybeNewTile[0]} ${maybeNewTile[1]}`);
		if (
			selectedMapTile[0] != maybeNewTile[0] ||
			selectedMapTile[1] != maybeNewTile[1]
		) {
			selectedMapTile = maybeNewTile;
			let mode = mouseMode.get();
			if (mode.major === "SELECTION") {
				clickBounds.count(maybeNewTile);
				let bounds = clickBounds.bounds();
				draw();

				// this doesn't quite work, it flickers a ton.
				// need to make another class that is handling this OR NOTHING
				// in the drawTile
				context.fillStyle = "blue";
				context.globalAlpha = 0.25;
				context.fillRect(
					bounds.x[0] * 24,
					bounds.y[0] * 24,
					bounds.x[1] * 24 + 24 - bounds.x[0] * 24,
					bounds.y[1] * 24 + 24 - bounds.y[0] * 24
				);
				context.fillStyle = pattern;
				context.globalAlpha = 1;
			}
		} else {
			draw();
		}
	};

	const handleSize = () => {
		const { top, left } = canvas.getBoundingClientRect();
		t = top;
		l = left;
	};
</script>

<!-- THIS IS STILL TRIGGERING on:keydown EVERYWHERE need to figure this out. -->
<!-- on:keydown={onKeyDown} -->

<svelte:window
	on:resize={handleSize}
	on:keydown={onKeyDown}
	on:keypress={onKeyHeld}
	on:keyup={onKeyUp}
/>

<div on:keydown={onKeyDown}>
	<canvas
		{width}
		{height}
		style:background
		bind:this={canvas}
		on:mousedown={handleStart}
		on:touchstart={handleStart}
		on:mouseup={handleEnd}
		on:touchend={handleEnd}
		on:mouseleave={handleExit}
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
