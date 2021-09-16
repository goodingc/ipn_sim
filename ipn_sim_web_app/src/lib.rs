pub mod bindings;
pub mod components;
pub mod event_html;
pub mod factories;
// pub mod movement_path;
pub mod sim_wrapper;
pub mod ts_append;
pub mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<components::app::App>();
    Ok(())
}
