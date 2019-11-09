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
    timer.request_animation_loop(Box::new(move |_delta_time| {
        for i in 0..pixels.len() {
            pixels[i] = random.gen::<f32>()*0.3;
        }
        framebuffer.render(&pixels)
    }));
}