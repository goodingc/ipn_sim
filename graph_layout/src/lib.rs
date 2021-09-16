use std::cell::RefCell;
use std::rc::Rc;

pub use cgmath;

pub use petgraph;

pub mod graph_layout;
pub mod vertex;

pub type Shared<T> = Rc<RefCell<T>>;

pub fn shared<T>(value: T) -> Shared<T> {
    Rc::new(RefCell::new(value))
}