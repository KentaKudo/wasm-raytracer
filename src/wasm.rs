use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render() -> Uint8Array {
    let arr = crate::raytracer::render();
    arr[..].into()
}
