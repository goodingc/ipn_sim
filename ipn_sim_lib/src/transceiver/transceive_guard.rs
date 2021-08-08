use crate::node::node::Node;
use dyn_clonable::clonable;

#[clonable]
pub trait TransceiveGuard: Clone {
    fn can_transceive(&self, transmitting_node: &Node, receiving_node: &Node) -> bool;
}
