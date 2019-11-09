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
    let (width,height) = framebuffer.dimensions();
    let mut pixels = vec![0.0_f32; width * height * 3];
    timer.request_animation_loop(create_callback_1(Box::new(move |_delta| {
        for i in 0..pixels.len() {
            pixels[i] = rng.gen::<f32>()*0.3;
        }
        framebuffer.render(&pixels)
    })))
}
