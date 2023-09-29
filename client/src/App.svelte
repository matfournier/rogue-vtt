<script lang="ts">
	import Game from "./lib/game/Game.svelte";
	import Login from "./lib/login/Login.svelte";
	import { tilesheets } from "./lib/domain/Tilesheet";
	import { loginStateStore } from "./lib/stores/UI";
	let promise = tilesheets();

	// login: create or join map
	// game: the game with the the associated ID attached
	let screen = "login";

	let state; // 0: login, 1: game load, 2: error
	loginStateStore.subscribe((s) => {
		state = s;
	});

	let render = true;
</script>

<main>
	{#if state.kind == "1"}
		{#await promise then tileSheet}
			<Game {tileSheet} {render} />
		{/await}
	{:else}
		<Login />
		{#if state.kind == "2"}
			<p>{state.text}</p>
			<p>error code: {state.code}</p>
		{/if}
	{/if}
</main>

<style>
</style>
