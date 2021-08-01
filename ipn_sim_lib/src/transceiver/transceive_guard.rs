use crate::node::Node;

pub trait TransceiveGuard {
    fn can_transceive(&self, transmitting_node: &Node, receiving_node: &Node) -> bool;
}
