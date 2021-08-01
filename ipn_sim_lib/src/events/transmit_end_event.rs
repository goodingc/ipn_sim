use crate::event::Event;
use crate::events::receive_data_event::ReceiveDataEvent;
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::node::Node;
use crate::utils::{Data, TimeMetric, C};
use cgmath::MetricSpace;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::Shared;


#[derive(Clone)]
pub struct TransmitEndEvent {
    pub node: Shared<Node>,
    pub data: Data,
}

impl Event for TransmitEndEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        let node_ref = self.node.borrow();
        let nodes = sim.nodes.take().unwrap();
        for receiving_node in &nodes {
            let receiving_node_ref = receiving_node.borrow();
            if &*node_ref as *const Node == &*receiving_node_ref as *const Node {
                continue;
            }
            let can_transceive = sim.nodes_can_transceive(&*node_ref, &*receiving_node_ref);
            if can_transceive {
                let flight_time =
                    (node_ref.position.distance(receiving_node_ref.position) / C) as TimeMetric;
                sim.add_event(
                    sim.time + flight_time,
                    ReceiveDataEvent {
                        node: Rc::clone(receiving_node),
                        data: self.data.clone(),
                    },
                )
            }
        }
        sim.nodes = Some(nodes);
    }

    fn is_internal() -> bool where Self: Sized {
        true
    }
}
