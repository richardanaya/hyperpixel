use js_ffi::*;

pub struct HyperPixel {
    instance:JSValue,
    fn_init:JSValue,
    fn_render:JSValue,
    fn_width:JSValue,
    fn_height:JSValue,
}

impl HyperPixel {
    /// Create an instance of HyperPixel from a selector to a canvas element.
    pub fn new(selector:&str) -> HyperPixel {
        let mut h = HyperPixel{
            instance: 0.0,
            fn_init:register("(selector)=>{
                    return new HyperPixel(document.querySelector(selector));
                }"),
            fn_render:register("(function(mem,ptr,length){
                let p = ptr/4;
                let pixelView = (new Float32Array(mem)).subarray(p,p+length);
                this.render(pixelView);
            })"),
            fn_width:register("(function(mem,ptr,length){
                return this.width;
            })"),
            fn_height:register("(function(mem,ptr,length){
                return this.height;
            })"),
        };
        h.init(selector);
        h
    }

    fn init(&mut self,selector:&str) {
        self.instance = call_1(UNDEFINED,self.fn_init,TYPE_STRING, to_js_string(selector))
    }

    /// Dimensions of the screen as tuple (width,height).
    pub fn dimensions(&self) -> (usize,usize) {
        let w = call_0(self.instance,self.fn_width) as usize;
        let h = call_0(self.instance,self.fn_height) as usize;
        (w,h)
    }

    /// Send a slice of float32 values to be pushed to the GPU and update the framebuffer immediately.
    pub fn render(&self,colors:&[f32]){
        call_3(self.instance,self.fn_render,TYPE_MEMORY, 0.0 as JSValue, TYPE_NUM, colors.as_ptr() as usize as JSValue, TYPE_NUM, colors.len() as f64 as JSValue);
    }
}