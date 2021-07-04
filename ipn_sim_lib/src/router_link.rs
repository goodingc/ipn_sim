use crate::utils::{Data, TimeMetric};
use crate::node::Node;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::events::transmit_start_event::TransmitStartEvent;

pub struct RouterLink<'a> {
    node: Rc<RefCell<Node>>,
    sim: &'a mut IpnSim,
}

impl<'a> RouterLink<'a> {
    pub fn new(node: &Rc<RefCell<Node>>, sim: &'a mut IpnSim) -> Self {
        Self {
            node: Rc::clone(node),
            sim,
        }
    }

    pub fn add_to_message_buffer(&self, data: Data) -> usize {
        self.node.borrow_mut().message_buffer.add_message(data)
    }

    pub fn add_to_transmit_buffer(&mut self, data: Data) {
        let transmit_start = self.node.borrow_mut().transceiver.add_to_buffer(data, self.sim.time);
        self.sim.add_event(
            transmit_start,
            TransmitStartEvent {
                node: Rc::clone(&self.node)
            },
        );
    }
}