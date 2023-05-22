<script lang="ts">
	import MapCanvas from "./lib/MapCanvas.svelte";
	import { tilesheets } from "./lib/domain/Tilesheet";
	let promise = tilesheets();
</script>

<main>
	{#await promise then tileSheet}
		<MapCanvas {tileSheet} />
	{/await}
</main>

<style>
</style>
