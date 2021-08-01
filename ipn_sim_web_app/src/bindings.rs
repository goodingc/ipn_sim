use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/ts/bindings.ts")]
extern "C" {
    pub fn setup(data: JsValue);

    pub fn tick(data: JsValue);

    #[wasm_bindgen(js_name = "getCameraPosition")]
    pub fn get_camera_position() -> JsValue;
}
