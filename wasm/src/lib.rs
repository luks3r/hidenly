mod hidenly;
mod utils;

use js_sys::{JsString, Uint8Array};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen]
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn decode(input: JsString) -> JsString {
    if let Some(i) = input.as_string() {
        JsString::from(hidenly::decode(i.as_str()))
    } else {
        input
    }
}

#[wasm_bindgen]
pub fn encode(input: JsString, secret: JsString) -> JsString {
    match (input.as_string(), secret.as_string()) {
        (Some(i), Some(s)) => JsString::from(hidenly::encode(i.as_str(), s.as_str())),
        _ => input,
    }
}

#[wasm_bindgen]
pub fn encode_binary(input: JsString, secret: Uint8Array) -> JsString {
    match (input.as_string(), secret.to_vec()) {
        (Some(i), s) => JsString::from(hidenly::encode_binary(i.as_str(), s)),
        _ => input,
    }
}

#[wasm_bindgen]
pub fn decode_binary(input: JsString) -> Uint8Array {
    match input.as_string() {
        Some(i) => Uint8Array::from(hidenly::decode_binary(i.as_str()).as_slice()),
        _ => Uint8Array::new_with_length(0),
    }
}