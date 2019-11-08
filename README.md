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

# With Rust?

see the demo [here](https://richardanaya.github.com/examples/helloworld/index.html)

```toml
[dependencies]
hyperpixel = "0.0.0"
```
```rust
use hyperpixel::*;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::Rng;

const WIDTH:usize = 160;
const HEIGHT:usize = 144;

#[no_mangle]
pub fn main() -> () {
    let mut rng: StdRng = SeedableRng::from_seed([1, 2, 3, 4, 1, 2, 3, 4,1, 2, 3, 4, 1, 2, 3, 4,1, 2, 3, 4, 1, 2, 3, 4,1, 2, 3, 4, 1, 2, 3, 4]);

    let h = HyperPixel::new("#screen");
    let mut pixels = vec![0.0_f32; WIDTH * HEIGHT * 3];
    for i in 0..pixels.len() {
        pixels[i] = rng.gen();
    }
    h.render(&pixels)
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
