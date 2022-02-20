use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(_: i64, _: i64) -> Uint8ClampedArray {
    let arr = crate::raytracer::render();
    arr[..].into()
}
