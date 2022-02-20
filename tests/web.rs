//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_render() {
    let got = wasm_raytracer::render(256, 256).unwrap();
    let want = 256 * 256 * 4;
    assert_eq!(got.length(), want);
}
