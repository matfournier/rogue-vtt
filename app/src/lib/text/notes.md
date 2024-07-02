


idea: 
  - players can be letters (upper case).  They get one color 
      enemies are lowercase same colour as player 

  - enemies all get another colour 
  - general lower case 
    - big bads can get an uppercase letter or @? 


  - capital letters are for players 
  - az are for henchman and enemies ? 

For speed: we can use the following? 

but drawing a few bits of text might be fine? 

https://developer.mozilla.org/en-US/docs/Web/API/createImageBitmap


Color picker is here -> 

https://www.npmjs.com/package/svelte-awesome-color-picker


// rendering text is like so.  remember you have to change the fillstyle back to pattern

```js
		context.font = "18pt Monospace";
		context.fillStyle = "grey";
		context.fillText("a", 6, 18);
		context.fillStyle = pattern;
```

// idea when enemies / players stack: they turn into numbers