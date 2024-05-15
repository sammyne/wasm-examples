//use wasm_bindgen::prelude::*;

//#[wasm_bindgen]
#[derive(Debug)]
#[repr(C)]
pub struct Greeting {
    pub a: u32,
    pub b: bool,
}

#[no_mangle]
pub extern "C" fn pass_struct(v: Greeting) -> u32 {
    if v.b {
        v.a + 1
    } else {
        v.a + 2
    }
}

#[no_mangle]
pub extern "C" fn return_struct(a: u32, b: bool) -> Greeting {
    Greeting { a, b }
}
