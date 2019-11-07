# hyperpixel

extremely fast pixel framebuffer using webgl

[demo](https://richardanaya.github.com/hyperpixel)

```javascript
var hp = new HyperPixel(document.getElementById("container"));
function update(){
  window.requestAnimationFrame(update);
  for(var i = 0 ; i < hp.height*hp.width*3; i++){
    hp.colors[p] = Math.random();
  }
  hp.render();
}
update();
```
