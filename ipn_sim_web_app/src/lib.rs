pub mod components;
pub mod bindings;
pub mod sim_wrapper;
pub mod factories;
pub mod ts_append;
pub mod utils;


use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<components::app::App>();
    Ok(())
}