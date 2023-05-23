<script lang="ts">
	import Game from "./lib/game/Game.svelte";
	import { tilesheets } from "./lib/domain/Tilesheet";
	let promise = tilesheets();
</script>

<main>
	{#await promise then tileSheet}
		<Game {tileSheet} />
	{/await}
</main>

<style>
</style>
