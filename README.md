# hyperpixel

<a href="https://docs.rs/hyperpixel"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

An extremely fast pixel framebuffer using webgl via ThreeJS

*Why is this fast?* 

Rendering literally just takes a Float32Array subview of wasm's memory and pushes it to the GPU via webgl to be rendered by an uber minimal shader for particles.

see the demo [here](https://richardanaya.github.com/hyperpixel)

```html
<canvas width="160" height="144" id="screen"></canvas>
<script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/109/three.min.js"></script>
<script src="https://cdn.jsdelivr.net/gh/richardanaya/hyperpixel@v0.0.3/hyperpixel.js"></script>
<script>
  var hp = new HyperPixel(document.getElementById("screen"));
  function update(){
    window.requestAnimationFrame(update);
    for(var i = 0 ; i < hp.height*hp.width*3; i++){
      hp.colors[i] = Math.random()*.3;
    }
    hp.render();
  }
  update();
</script>
```

# With Rust?

see the demo [here](https://richardanaya.github.com/hyperpixel/examples/static/index.html)

```toml
[dependencies]
hyperpixel = "0.1"
web_timer = "0.1" # for interacting with timing functions in browser
web_random = "0.0" # for generating random numbers efficiently
```
```rust
use hyperpixel::*;
use web_timer::*;
use web_random::*;

#[no_mangle]
pub fn main() -> () {
    let timer = Timer::default();
    let mut random = Random::default();
    let framebuffer = HyperPixel::new("#screen");
    let (width,height) = framebuffer.dimensions();
    let mut pixels = vec![0.0; width * height * 3];
    timer.request_animation_loop(Box::new(move |_delta| {
        for i in 0..pixels.len() {
            pixels[i] = random.gen::<f32>()*0.3;
        }
        framebuffer.render(&pixels)
    }));
}
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
