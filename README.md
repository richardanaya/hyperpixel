# hyperpixel

extremely fast pixel framebuffer using webgl via ThreeJS

see the demo [here](https://richardanaya.github.com/hyperpixel)

```html
<canvas width="160" height="144" id="screen"></canvas>
<script src="https://cdn.jsdelivr.net/gh/richardanaya/hyperpixel/hyperpixel.js"></script>
<script src="hyperpixel.js"></script>
<script>
  var hp = new HyperPixel(document.getElementById("screen"));
  function update(){
    window.requestAnimationFrame(update);
    for(var i = 0 ; i < hp.height*hp.width*3; i++){
      hp.colors[i] = Math.random();
    }
    hp.render();
  }
  update();
</script>
```
