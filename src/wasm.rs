use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(_: u16, _: u16) -> Uint8ClampedArray {
    let arr = crate::raytracer::render();
    arr[..].into()
}
