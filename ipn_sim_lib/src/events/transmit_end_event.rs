use crate::node::Node;
use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::{Data, TimeMetric, C};
use crate::event::Event;
use crate::ipn_sim::ipn_sim::IpnSim;
use cgmath::MetricSpace;
use crate::events::receive_data_event::ReceiveDataEvent;

pub struct TransmitEndEvent {
    pub node: Rc<RefCell<Node>>,
    pub data: Data
}

impl Event for TransmitEndEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        let node_ref = self.node.borrow();
        let mut nodes = sim.nodes.take().unwrap();
        for receiving_node in &nodes {
            let receiving_node_ref = receiving_node.borrow();
            if &*node_ref as *const Node == &*receiving_node_ref as *const Node {
                continue;
            }
            let can_transceive = node_ref.transceiver.guard.can_transceive(
                &*node_ref,
                &*receiving_node_ref,
            );
            if can_transceive {
                let flight_time = (
                    node_ref.position.distance(receiving_node_ref.position) / C
                ) as TimeMetric;
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
}