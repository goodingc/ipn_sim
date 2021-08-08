use cgmath::MetricSpace;

use crate::node::node::Node;
use crate::transceiver::transceive_guard::TransceiveGuard;
use crate::utils::SpaceMetric;

#[derive(Clone)]
pub struct SimpleTransceiveGuard {
    transceive_distance: SpaceMetric,
}

impl SimpleTransceiveGuard {
    pub fn new(transceive_distance: SpaceMetric) -> Self {
        Self {
            transceive_distance,
        }
    }
}

impl TransceiveGuard for SimpleTransceiveGuard {
    fn can_transceive(&self, transmitting_node: &Node, receiving_node: &Node) -> bool {
        transmitting_node.position.distance(receiving_node.position) <= self.transceive_distance
    }
}
