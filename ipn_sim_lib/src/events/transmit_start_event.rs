use crate::event::Event;
use crate::events::transmit_end_event::TransmitEndEvent;
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::node::node::Node;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::Shared;


#[derive(Clone)]
pub struct TransmitStartEvent {
    pub node: Shared<Node>,
}

impl Event for TransmitStartEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        let mut node_ref = self.node.borrow_mut();
        let data = node_ref.transceiver.pop_head_data();
        sim.add_event(
            sim.time + node_ref.transceiver.get_transmit_time(&data),
            TransmitEndEvent {
                node: Rc::clone(&self.node),
                data,
            },
        )
    }

    fn is_internal() -> bool where Self: Sized {
        true
    }
}
