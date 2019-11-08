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
