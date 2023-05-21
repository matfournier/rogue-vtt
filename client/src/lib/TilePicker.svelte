<script>
  import { getContext } from 'svelte';	
  import PopupLong from './PopupLong.svelte';

  const { open } = getContext('simple-modal');
	
	let opening = false;
	let opened = false;
	let closing = false;
	let closed = false;

	const showPopupLong = () => {
		open(PopupLong, { message: "It's a popup with long text!" });
	};
	

</script>

<section>
	<!-- <button on:click={showPopup}>Show a popup!</button>
	<br/> -->
	<button on:click={showPopupLong}>Show a popup with long text!</button>
	<br/>
	<!-- <button on:click={showPopupCustom}>Show a customized popup and listen to events!</button>
	<br/>
	<button on:click={showDialog}>Show a dialog!</button> -->

	<!-- <div id="state">
		{#if opening}
			<p>opening modal...</p>
		{:else if opened}
			<p>opened modal!</p>
		{:else if closing}
			<p>closing modal...</p>
		{:else if closed}
			<p>closed modal!</p>
		{/if}
	</div> -->
</section>

<style>
	section {
		padding-top: 0.5em;
	}
	
	#state {
		position: absolute;
		top: 0;
		right: 0;
		opacity: 0.33;
		font-size: 0.8em;
	}
</style>