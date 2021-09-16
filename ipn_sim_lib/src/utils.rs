use std::cell::RefCell;
use std::rc::Rc;

use bit_vec::BitVec;
use wasm_bindgen::prelude::*;

pub type TimeMetric = u64;
pub type SpaceMetric = f64;

pub type Data = Box<[u8]>;

pub type NodeId = u16;
pub type MessageId = u32;

pub const C: SpaceMetric = 299_792_458e-9;
pub const G: SpaceMetric = 6.674e-11;

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

pub type Shared<T> = Rc<RefCell<T>>;

pub fn shared<T>(value: T) -> Shared<T> {
    Rc::new(RefCell::new(value))
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::utils::Shared;

    trait X {}

    struct A;

    impl X for A {}

    fn a(a: &Shared<impl X + 'static>, x: &mut Vec<Shared<dyn X>>) {
        x.push(a.clone());
    }

    #[test]
    fn test() {
        let mut x: Vec<Shared<dyn X>> = vec![];

        let b = Rc::new(RefCell::new(A));

        a(&b, &mut x);
    }
}