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
	import Palette from "../popups/Palette.svelte";
	import { bind } from "../Modal.svelte";

	import Modal from "../Modal.svelte";
	import { selectedTileStore } from "../stores/UI";
	import { entityEventStore } from "../stores/UI";
	import { MouseMode } from "./MouseMode";
	import { EntityState, EntityType } from "../domain/EntityRenderer";
	import {
		DrawHandler,
		EventType,
		MoveHandler,
		PlaceHandler,
		UActionType,
		ViewHandler,
	} from "./Interaction";
	import { Camera } from "./Camera";
	import { LocalEventSystem } from "../domain/EventSystem";
	import Sidebar from "../Sidebar.svelte";

	const tileSize = 24;
	const mapSize = [200, 200];
	const cameraDimensions = [56, 32];

	export let width = cameraDimensions[0] * tileSize;
	export let height = cameraDimensions[1] * tileSize;
	export let tileSheet;

	export let background = "#fff";

	let pattern;

	let paletteSelected;
	let interfaceHandler;

	let stores;

	let camera;
	let canvas;
	let context;
	let selectedMapTile;
	let mode;
	let entities;
	let es;
	let t, l;
	let mapFocus = true;
	let serverCall;

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

	onMount(async () => {
		selectedMapTile = [0, 0];
		camera = new Camera(
			[selectedMapTile[0], selectedMapTile[1]],
			cameraDimensions[0],
			cameraDimensions[1],
			mapSize[0],
			mapSize[1]
		);
		viewHandlerFactory("RESET");

		let map = new MapState(mapSize[0], mapSize[1], tileSheet, camera);
		entities = new EntityState(camera);
		es = new LocalEventSystem(map, entities, camera);
		// TODO: this default canvas stuff should move somewhere else.
		let defaultTile = tileSheet.dungeon.tiles[101];
		context = canvas.getContext("2d");
		pattern = context.createPattern(defaultCanvas(defaultTile), "repeat");
		context.fillStyle = pattern;
		context.font = "18pt Monospace";
		handleSize();
		draw();
		stores = {
			selected: selectedTileStore.subscribe((value) => {
				paletteSelected = value;
				if (interfaceHandler !== undefined) {
					interfaceHandler.update(value);
				}
			}),
			entityEvents: entityEventStore.subscribe((events) => {
				handleEvent(events);
			}),
		};

		fetch("http://localhost:3000/hello")
			.then((response) => response.json())
			.then((result) => {
				serverCall = result.name;
				console.log(result);
			});
	});

	const teardown = () => {
		stores.forEach((unsub, k) => unsub());
	};

	onDestroy(teardown);

	function onKeyDown(e) {
		interfaceHandler.onKeyDown(e);
	}

	function onKeyUp(e) {
		// if (mapfocus) // need to bring this back when dealing with PLACE
		// possible w/ some other events?
		// can you do ignore keyboard event -> popup -> resume keyboard event as a sequence of three events?
		// you notice this when placing glyphs and then accidentally triggering some other mode.
		if (!e.shiftKey) {
			switch (e.keyCode) {
				case 68: // d == draw mode
					viewHandlerFactory("DRAW");
					break;
				case 86: // v == view mode
					viewHandlerFactory("RESET");
					break;
				case 80: // p == place mode
					viewHandlerFactory("PLACE");
					break;
				default:
					handleEvent(interfaceHandler.onKeyUp(e));
					draw();
					break;
			}
		}
	}

	function onKeyHeld(e) {
		interfaceHandler.onKeyPressed(e);
	}

	const handleStart = () => {
		handleEvent(interfaceHandler.onClick(selectedMapTile));
	};

	const modeReset = () => {};

	const handleEvent = (e) => {
		if (e !== undefined && e[0] !== undefined) {
			let gameEvents = [];
			e.forEach((ee) => {
				if (ee.type === EventType.GAME) {
					gameEvents.push(ee);
				} else {
					handleDisplayEvent(ee);
				}
			});
			gameEvents.forEach((e) => es.event(e));
			draw();
		}
	};

	const handleDisplayEvent = (e) => {
		if (e.action.kind === UActionType.PopupDungeon) {
			modal.set(
				bind(Palette, {
					tilesheet: tileSheet.dungeon,
					height: 6 * 24,
					icons: tileSheet.icon,
					sheetName: "dungeon",
				})
			);
		} else if (e.action.kind === UActionType.PopupFeature) {
			modal.set(
				bind(Palette, {
					tilesheet: tileSheet.feature,
					height: 2 * 24,
					icons: tileSheet.icon,
					sheetName: "feature",
				})
			);
		} else if (e.action.kind === UActionType.PlaceToken) {
			mapFocus = false;
			modal.set(
				bind(EntityForm, {
					entities: entities,
					xy: e.action.xy,
					callback: () => {
						mapFocus = true;
						draw();
					},
				})
			);
		} else if (e.action.kind === UActionType.MoveEntityStart) {
			interfaceHandler = new MoveHandler(
				camera,
				tileSheet.icon,
				selectedMapTile,
				e.action
			);
		} else if (e.action.kind === UActionType.Reset) {
			viewHandlerFactory("RESET");
		}
	};

	const draw = () => {
		context.fillRect(0, 0, context.canvas.width, context.canvas.height);
		es.render(context);
		interfaceHandler.render(context);
	};

	const handleEnd = () => {
		handleEvent(interfaceHandler.onEnd(selectedMapTile));
		draw();
	};

	const handleExit = () => {
		handleEvent(interfaceHandler.onLeave());
		draw();
	};

	const viewHandlerFactory = (s) => {
		if (s === "RESET") {
			interfaceHandler = new ViewHandler(
				camera,
				tileSheet.icon,
				selectedMapTile
			);
			mode = "VIEW";
		} else if (s === "DRAW") {
			interfaceHandler = new DrawHandler(
				paletteSelected,
				camera,
				tileSheet.icon,
				selectedMapTile
			);
			mode = "DRAW";
		} else if (s === "PLACE") {
			interfaceHandler = new PlaceHandler(
				camera,
				tileSheet.icon,
				selectedMapTile
			);
			mode = "PLACE TOKEN";
		}
	};

	const handleMove = ({ offsetX: x1, offsetY: y1 }) => {
		let canvasTile = Grid.getTileCoords(x1, y1);
		selectedMapTile = [
			canvasTile[0] + camera.leftX,
			canvasTile[1] + camera.topY,
		]; //
		interfaceHandler.onMove(selectedMapTile);
		draw();
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

<div class="main">
	<div>
		<canvas
			class="game"
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
	<div class="sidebar">
		<Sidebar />
		<!-- <p>some content</p> -->
	</div>
</div>
<Toolbar bind:tilePos={selectedMapTile} bind:modeString={mode} />
<Modal show={$modal}>
	<!-- <TilePicker {tileSheet} /> -->
</Modal>

<!-- // need to figure out how to share this better. -->
<Modal>
	<!-- <EntityPicker {entities} /> -->
</Modal>

<p>{serverCall}</p>

<style>
	div.main {
		display: flex;
		width: 100;
	}
	div.sidebar {
		background-color: black;
		flex-grow: 1;
		padding: 0.5rem;
	}
</style>
