use std::cell::RefCell;
use crate::node::Node;
use std::rc::Rc;
use crate::utils::Data;
use crate::event::Event;
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::router_link::RouterLink;

pub struct ReceiveDataEvent {
    pub node: Rc<RefCell<Node>>,
    pub data: Data
}

impl Event for ReceiveDataEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        self.node
            .borrow_mut()
            .router
            .as_mut()
            .unwrap()
            .on_data_received(
                &mut RouterLink::new(&self.node, sim),
                self.data
            )
    }
}