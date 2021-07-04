use crate::node::Node;
use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::Data;
use crate::event::Event;
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::router_link::RouterLink;
use crate::utils;

pub struct CreateMessageEvent {
    pub node: Rc<RefCell<Node>>,
    pub data: Data,
}

impl Event for CreateMessageEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        let mut router = self.node.borrow_mut().router.take().unwrap();
        router.on_message_created(
            &mut RouterLink::new(&self.node, sim),
            self.data,
        );
    }
}