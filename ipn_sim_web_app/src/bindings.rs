use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="/ts/bindings.ts")]
extern {
    pub fn setup();
}