use std::cell::RefCell;
use std::rc::Rc;

use crate::event::Event;
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::node::Node;
use crate::router_link::RouterLink;
use crate::utils::Data;
use crate::utils::Shared;


#[derive(Clone)]
pub struct ReceiveDataEvent {
    pub node: Shared<Node>,
    pub data: Data,
}

impl Event for ReceiveDataEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        let mut node_ref = self.node.borrow_mut();
        let mut router = node_ref.router.take().unwrap();
        router.on_data_received(
            &mut RouterLink::new(&mut node_ref, &self.node, sim),
            self.data,
        );
        node_ref.router = Some(router);
    }

    fn is_internal() -> bool where Self: Sized {
        true
    }
}
