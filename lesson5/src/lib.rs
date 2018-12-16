#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../domUtils")]
extern "C" {
    fn appendStringToBody(s: &str);
}

#[wasm_bindgen]
pub extern "C" fn run() {
    appendStringToBody("Hello World");
}
