<script lang="ts">
	import Game from "./lib/game/Game.svelte";
	import { tilesheets } from "./lib/domain/Tilesheet";
	let promise = tilesheets();

	let render = true;
</script>

<main>
	{#await promise then tileSheet}
		<Game {tileSheet} {render} />
	{/await}
</main>

<style>
</style>
