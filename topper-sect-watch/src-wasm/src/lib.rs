use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;
pub mod log_parser;
pub mod observations;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    unsafe {
        log("Hello, wasm-game-of-life!");
    }
}
