pub mod components;
pub mod scenario;
pub mod scenario_grid_wrapper;
pub mod router_details;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<components::app::App>();
    Ok(())
}
