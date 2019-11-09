# hyperpixel

<a href="https://docs.rs/hyperpixel"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

An extremely fast pixel framebuffer using webgl via ThreeJS

*Why is this fast?* 

Rendering literally just takes subview of wasm's memory and pushes it to the GPU via webgl to be rendered by an uber minimal shader for particles.

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
web_timer = "0.0.2" # for interacting with timing functions in browser
rand = "0.7.2" # for getting a psuedo random number generator
js_ffi = "0.1" # for creating the callback that gets sent into web_timer's `request_animation_loop`
```
```rust
use hyperpixel::*;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use web_timer::*;
use js_ffi::*;

#[no_mangle]
pub fn main() -> () {
    let timer = Timer::default();
    let mut rng:StdRng = SeedableRng::from_seed([
        1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2,
        3, 4,
    ]);
    let framebuffer = HyperPixel::new("#screen");
    let (width,height) = framebuffer.dimensions()
    let mut pixels = vec![0.0_f32; width * height * 3];
    timer.request_animation_loop(create_callback_1(Box::new(move |delta_time| {
        for i in 0..pixels.len() {
            pixels[i] = rng.gen::<f32>()*.3;
        }
        framebuffer.render(&pixels)
    })))
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
