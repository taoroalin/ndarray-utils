mod utils;

use half::f16;
use std::iter::zip;
use std::mem::transmute;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, ndarray-utils!");
}

#[wasm_bindgen]
pub fn float16_to_float32(float16s: &[i16], float32s_out: &mut [f32]) {
    let as_float16: &[f16] = unsafe { transmute(float16s) };
    for pair in zip(as_float16, float32s_out.iter_mut()) {
        *pair.1 = (pair.0).to_f32()
    }
}
