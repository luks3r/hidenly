#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm::greet;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(2 + 2, 4);
}
