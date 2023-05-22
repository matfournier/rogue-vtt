<!-- <script>
    export let message;
  
  </script>
  
  <style>
    h1 {
          font-size: 2rem;
          text-align: center;
      }
      
  </style>
  
  <h1>🎉 {message} 🍾</h1> -->

<script>
	import { onMount } from "svelte";

	let tileSize = 24;

	export let width = 32 * tileSize;
	export let height = 6 * tileSize;
	export let color = "#333";
	export let background = "#fff";
	export let tilesheet;

	let canvas;
	let context;
	let isDrawing;
	let start;

	let t, l;

	onMount(() => {
		context = canvas.getContext("2d");
		context.lineWidth = 3;
		tilesheet.render(context);
	});

	$: if (context) {
		context.strokeStyle = color;
	}

	const handleStart = ({ offsetX: x, offsetY: y }) => {
		if (color === background) {
			context.clearRect(0, 0, width, height);
		} else {
			isDrawing = true;
			start = { x, y };
		}
	};

	const handleMove = ({ offsetX: x1, offsetY: y1 }) => {
		if (!isDrawing) return;

		const { x, y } = start;
		context.beginPath();
		context.moveTo(x, y);
		context.lineTo(x1, y1);
		context.closePath();
		context.stroke();

		start = { x: x1, y: y1 };
	};
</script>

<canvas
	{width}
	{height}
	style:background
	bind:this={canvas}
	on:mousedown={handleStart}
	on:touchstart={(e) => {
		const { clientX, clientY } = e.touches[0];
		handleStart({
			offsetX: clientX - l,
			offsetY: clientY - t,
		});
	}}
	on:mousemove={handleMove}
	on:touchmove={(e) => {
		const { clientX, clientY } = e.touches[0];
		handleMove({
			offsetX: clientX - l,
			offsetY: clientY - t,
		});
	}}
/>
