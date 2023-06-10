<script>
	import { onDestroy } from "svelte";
	import { onMount } from "svelte";
	import { Grid, SquareCounter } from "../domain/Grid";
	import Toolbar from "../Toolbar.svelte";
	import { MapState } from "../domain/DungeonMap";
	import TilePicker from "../popups/TilePicker.svelte";
	import EntityPicker from "../popups/EntityPicker.svelte";
	import { modal } from "../stores/UI";
	import EntityForm from "../popups/EntityForm.svelte";
	import { bind } from "../Modal.svelte";

	import Modal from "../Modal.svelte";
	import { selectedTileStore } from "../stores/UI";
	import { MouseMode } from "./MouseMode";
	import { EntityState, EntityType } from "../domain/EntityRenderer";
	import { ViewHandler } from "./Interaction";

	// https://svelte.dev/repl/434e0b14546747688401e8808c060a23?version=3.47.0

	let tIdx = 97;
	$: tileIndex = tIdx;

	const tileSize = 24;
	let dimensions = [50, 25];

	const interfaceHandler = new ViewHandler();

	export let width = dimensions[0] * tileSize;
	export let height = dimensions[1] * tileSize;
	export let tileSheet;

	export let background = "#fff";

	let pattern;

	let paletteSelected;

	const mouseMode = new MouseMode();
	let clickBounds; // when someone clicks, decide if you should try a single or multiple tile

	const stores = {
		selected: selectedTileStore.subscribe((value) => {
			paletteSelected = value;
		}),
	};

	let canvas;
	let context;
	let selectedMapTile;
	let map;
	let entities;
	let t, l;
	let mapFocus = true;

	// has modal from a text box I think.
	//	https://svelte.dev/repl/b95ce66b0ef34064a34afc5c0249f313?version=3.59.1

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
		entities = new EntityState();
		let defaultTile = tileSheet.dungeon.tiles[101];
		context = canvas.getContext("2d");
		pattern = context.createPattern(defaultCanvas(defaultTile), "repeat");
		context.fillStyle = pattern;
		context.font = "18pt Monospace";
		handleSize();
		draw();
	});

	const teardown = () => {
		stores.forEach((unsub, k) => unsub());
	};

	onDestroy(teardown);

	function onKeyDown(e) {
		if (mapFocus) {
			console.log(e.key);
			switch (e.keyCode) {
				case 27: // esc
					modeReset();
					draw();
					break;
			}
		}
	}

	function onKeyUp(e) {
		interfaceHandler.onKeyUp(e);
		if (mapFocus) {
			switch (e.keyCode) {
				case 8: // delete
					// when minor mode has a selected entity, delete it?
					// you have the entity id to delete we could probably implement this
					break;
				case 49: // 1
					// todo launch the floor/wall picker
					break;
				case 50: // 2
					// todo launch the feature pickler
					break;
				case 88: // esc
					mouseMode.reset();
					mapFocus = true;
					break;
				case 83: // s
					mouseMode.setSelection();
					draw();
					break;
				case 65: // a
					mapFocus = false;
					modal.set(
						bind(EntityForm, {
							entities: entities,
							xy: selectedMapTile,
							entityType: EntityType.PLAYER,
							callback: () => {
								mapFocus = true;
								draw();
							},
						})
					);
					break;
				case 69: // e
					mapFocus = false;
					modal.set(
						bind(EntityForm, {
							entities: entities,
							xy: selectedMapTile,
							entityType: EntityType.NPC,
							callback: () => {
								mapFocus = true;
								draw();
							},
						})
					);
					break;
			}
		}
	}

	function onKeyHeld(e) {
		if (mapFocus) {
			switch (e.keyCode) {
				case 88: // x
					if (e.shiftKey) {
						mouseMode.setMinorClear();
					} else {
						mouseMode.setMinorClearAll();
					}
					break;
			}
		}
	}

	const handleStart = () => {
		if (mouseMode.get().major !== "SELECTION") {
			clickBounds = new SquareCounter(selectedMapTile);
			if (mouseMode.get().major === "NONE") {
				mouseMode.setRange();
			}
		} else {
			// you are in selection mode

			if (mouseMode.get().minor === "TARGET") {
				// we are placing a tile
			} else {
				let stack = entities.list(
					selectedMapTile[0],
					selectedMapTile[1]
				);
				// we are selecting a tile
				// if nothing there -> mode reset
				// if we click on something
				//   if there is one entity
				//   if there is more than one entity open up dialog box to select which one
			}
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
		entities.render(context);

		if (mouseMode.get().major !== "SELECTION") {
			tileSheet.icon.renderCursor(context, selectedMapTile);
		} else {
			tileSheet.icon.renderSelectionCursor(context, selectedMapTile);
		}

		context.font = "18pt Monospace";
		context.fillStyle = "grey";
		context.fillText("A", 40 * 24, 18 * 24);
		context.fillText("B", 42 * 24, 20 * 24);

		context.fillStyle = "yellow";

		context.fillText("g", 44 * 24, 19 * 24);
		context.fillText("g", 42 * 24, 21 * 24);
		context.fillText("g", 40 * 24, 21 * 24);
		context.fillText("g", 42 * 24, 22 * 24);

		context.fillStyle = pattern;
	};

	const handleEnd = () => {
		// console.log(tiles);
		let mode = mouseMode.get();
		if (mode.major === "RANGE") {
			let tiles = clickBounds.tilesLim();
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
		selectedMapTile = Grid.getTileCoords(x1, y1);
		draw();
		let mode = mouseMode.get();
		if (mode.major === "RANGE") {
			clickBounds.lim(selectedMapTile);
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
<Modal show={$modal}>
	<TilePicker {tileSheet} />
</Modal>

<Modal>
	<EntityPicker {entities} />
</Modal>
