<script>
	import { onMount } from 'svelte'
    import {Logic } from '../lib/logic/logic'	
    import tileSheetLoc from '../assets/vttwall.png'
    import { xlink_attr } from 'svelte/internal';

    let tileSize = 32
    let dimensions = [32, 20]

	export let width = dimensions[0] * tileSize 
	export let height = dimensions[1]  * tileSize 

	export let color = '#333'
	export let background = '#fff'


    let tiles

	let canvas
	let context
	let isDrawing
	let start	
	let t, l
    let tileSheet

    let position
	
	onMount(() => {
    
        tiles = Array(dimensions[0] * dimensions[1])

        tileSheet = new Image()  
        tileSheet.src = (tileSheetLoc)
        context = canvas.getContext('2d')
	    context.lineWidth = 3
	    handleSize()
        
	})
	
	$: if(context) {
			context.strokeStyle = color
	}
	
	function onKeyDown(e) {
        console.log(e.key)
		 switch(e.keyCode) {
			 case 88: // x
				 console.log("x!")
                 // deletes our tiles from our tile map
                 for (let idx in tiles) {
                    delete tiles[idx]
                 }
                // this clears everything on a canvas 
                 context.clearRect(0, 0, context.canvas.width, context.canvas.height)
                break;

		 }
	}

	const handleStart = (({ offsetX: x, offsetY: y }) => { 
        let [xx, yy] = Logic.getTileCoords(x, y)
        console.log(`x: ${xx}, y: ${yy}`)
        tiles[Logic.tileIndex(xx, yy)] = {x: xx, y: yy, id: 0}
        drawTile()
		if(color === background) {
			context.clearRect(0, 0, width, height)
		} else {
			isDrawing = true
			start = { x, y }
		}
	})

    const drawTile = () => {
        console.log(tiles)
        tiles.forEach((tile) =>  {
            console.log(`tileL ${tile}`)
       context.drawImage(
            tileSheet,
            tile.id * 32,
            tile.id * 32,
            tileSize,
            tileSize,
            tile.y * 32,
            tile.x * 32,
            tileSize,
            tileSize
         );
        }
        )


        // let [xx, yy] = Logic.getTileCoords(x, y)
        // let pos = 0
        // context.drawImage(
        //     tileSheet,
        //     pos * 32,
        //     pos * 32,
        //     tileSize,
        //     tileSize,
        //     yy * 32,
        //     xx * 32,
        //     tileSize,
        //     tileSize
        //  );
    }
	
	const handleEnd = () => { isDrawing = false }
	const handleMove = (({ offsetX: x1, offsetY: y1 }) => {
        position = Logic.getTileCoords(x1, y1)
        // console.log(position)
		// if(!isDrawing) return
		
		// const { x, y } = start
		// context.beginPath()
		// context.moveTo(x, y)
		// context.lineTo(x1, y1)
		// context.closePath()
		// context.stroke()
		
		// start = { x: x1, y: y1 }
	})
	
	const handleSize = () => {
		const { top, left } = canvas.getBoundingClientRect()
		t = top
		l = left
	}
</script>

<!-- need to put this on a div wrapping the canvas somehow -->
<svelte:window on:resize={handleSize}  on:keydown|preventDefault={onKeyDown} 
/>

<canvas
				{width}
				{height}
				style:background
				bind:this={canvas} 
				on:mousedown={handleStart}	
				on:touchstart={e => {
					const { clientX, clientY } = e.touches[0]
					handleStart({
						offsetX: clientX - l,
						offsetY: clientY - t
					})
				}}	
				on:mouseup={handleEnd}				
				on:touchend={handleEnd}				
				on:mouseleave={handleEnd}
				on:mousemove={handleMove}
				on:touchmove={e => {
					const { clientX, clientY } = e.touches[0]
					handleMove({
						offsetX: clientX - l,
						offsetY: clientY - t
					})
				}}
				/>