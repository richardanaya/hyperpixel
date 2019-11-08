use js_ffi::*;

pub struct HyperPixel {
    instance:JSValue,
    fn_init:JSValue,
    fn_render:JSValue,
}

impl HyperPixel {
    pub fn new(selector:&str) -> HyperPixel {
        let mut h = HyperPixel{
            instance: 0.0,
            fn_init:register("(selector)=>{
                    return new HyperPixel(document.querySelector(selector));
                }"),
            fn_render:register("HyperPixel.prototype.render"),
        };
        h.init(selector);
        h
    }
    pub fn init(&mut self,selector:&str) {
        self.instance = call_1(UNDEFINED,self.fn_init,TYPE_STRING, to_js_string(selector))
    }

    pub fn render(&self,colors:&Vec<f32>){
        call_1(self.instance,self.fn_render,TYPE_F32_ARRAY, to_js_typed_array(colors).as_js_ptr());
    }
}