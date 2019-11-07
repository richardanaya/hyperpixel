# hyperpixel

extremely fast pixel framebuffer using webgl via ThreeJS

see the demo [here](https://richardanaya.github.com/hyperpixel)

```html
<canvas width="160" height="144" id="screen"></canvas>
<script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/109/three.min.js"></script>
<script src="https://cdn.jsdelivr.net/gh/richardanaya/hyperpixel/hyperpixel.js"></script>
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

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `hyperpixel` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
