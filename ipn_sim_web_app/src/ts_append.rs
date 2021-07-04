use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type Point3<T> = {
    x: T,
    y: T,
    z: T,
}

export type SpaceMetric = number
"#;
