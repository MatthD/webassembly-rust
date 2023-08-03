#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/wasmUtils.js")]
extern {
  fn appendToBody(s: &str);
}
#[wasm_bindgen]
pub fn run(){
  appendToBody("blabla");
}


