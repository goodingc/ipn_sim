mod components;
mod bindings;
mod sim_wrapper;

use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<components::app::App>();
    Ok(())
}