<script lang="ts">
	import Game from "./lib/game/Game.svelte";
	import Login from "./lib/login/Login.svelte";
	import { tilesheets } from "./lib/domain/Tilesheet";
	import { loginStateStore } from "./lib/stores/UI";
	import { LoginResultType } from "./lib/login/loginstate";
	let promise = tilesheets(); // TODO  move this to GameState

	// login: create or join map
	// game: the game with the the associated ID attached
	let screen = "login";
	let gameState;
	let errorText;

	let state = 0; // 0: NoResult, 1: game load, 2: error
	loginStateStore.subscribe((s) => {
		if (s.kind == LoginResultType.Load) {
			state = s.kind;
			gameState = s.game;
		} else if (s.kind === LoginResultType.Error) {
			state = s.kind;
			errorText = s.error;
		}
	});

	let render = true; // what is this for?
</script>

<main>
	{#if state == 1}
		{#await promise then tileSheet}
			<Game {tileSheet} {gameState} {render} />
		{/await}
	{:else}
		<Login />
		{#if state == 2}
			<p>ERROR: {errorText}</p>
		{/if}
	{/if}
</main>

<style>
</style>
