<script lang="ts">
	import MapCanvas from "./lib/MapCanvas.svelte";
	const background = "#000000";
</script>

<main>
	<!-- <MapCanvas {background} tileSheet={ts} /> -->
	<MapCanvas {background}/>

</main>

<style>
	/* :global(body) {
		margin: 0;
		padding: 0;
		min-height: 100vh;
		display: flex;
		justify-content: center;
		align-items: center;
		background: #e5e5e5;
	} */

	/* :global(.visually-hidden:not(:focus):not(:active)) {
		clip: rect(0 0 0 0);
		clip-path: inset(50%);
		height: 1px;
		width: 1px;
		overflow: hidden;
		position: absolute;
		white-space: nowrap;
	} */

	/* main {
		max-width: 300px;
		display: flex;
		flex-direction: column;
		gap: 0.5rem 0;
	}
	
	main :global(canvas) {
		align-self: center;
	}
	 */
</style>
