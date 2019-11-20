use js_ffi::*;

pub struct HyperPixel {
    instance: Option<JSObject>,
    fn_init: JSInvoker,
    fn_render: JSInvoker,
    fn_width: JSInvoker,
    fn_height: JSInvoker,
}

impl HyperPixel {
    /// Create an instance of HyperPixel from a selector to a canvas element.
    pub fn new(selector: &str) -> HyperPixel {
        let mut h = HyperPixel {
            instance: None,
            fn_init: js!((selector)=>{
                return new HyperPixel(document.querySelector(selector));
            }),
            fn_render: js!((function(mem,ptr,length){
                let p = ptr/4;
                let pixelView = (new Float32Array(mem)).subarray(p,p+length);
                this.render(pixelView);
            })),
            fn_width: js!((function(mem,ptr,length){
                return this.width;
            })),
            fn_height: js!((function(mem,ptr,length){
                return this.height;
            })),
        };
        h.init(selector);
        h
    }

    fn init(&mut self, selector: &str) {
        self.instance = Some(JSObject(self.fn_init.invoke_1(selector)))
    }

    /// Dimensions of the screen as tuple (width,height).
    pub fn dimensions(&self) -> (usize, usize) {
        let w = self.fn_width.call_0(self.instance.as_ref().unwrap()) as usize;
        let h = self.fn_height.call_0(self.instance.as_ref().unwrap()) as usize;
        (w, h)
    }

    /// Send a slice of float32 values to be pushed to the GPU and update the framebuffer immediately.
    pub fn render(&self, colors: &[f32]) {
        self.fn_render.call_3(
            self.instance.as_ref().unwrap(),
            WasmMemory,
            colors.as_ptr() as u32,
            colors.len() as u32,
        );
    }
}
