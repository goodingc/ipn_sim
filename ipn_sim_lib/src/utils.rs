use bit_vec::BitVec;
use wasm_bindgen::prelude::*;

pub type TimeMetric = u64;
pub type SpaceMetric = f64;

pub type Data = BitVec;

pub type NodeId = u32;

pub const C: f64 = 299_792_458e-9;


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log(s: &str) {
    println!("{}", s);
}