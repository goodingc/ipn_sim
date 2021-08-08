use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::event::Event;
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::node::node::Node;
use crate::router_link::RouterLink;
use crate::utils::{Data, NodeId, TimeMetric};
use crate::utils::Shared;

#[derive(Clone)]
pub struct CreateMessageEvent {
    pub node: Shared<Node>,
    pub destination: MessageDestination,
    pub payload: Data,
    pub ttl: Option<TimeMetric>,
}

impl Event for CreateMessageEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        let mut node_ref = self.node.borrow_mut();
        let mut router = node_ref.router.take().unwrap();
        router.on_message_created(
            &mut RouterLink::new(&mut node_ref, &self.node, sim),
            self.destination,
            self.payload,
            self.ttl,
        );
        node_ref.router = Some(router);
    }

    fn is_internal() -> bool where Self: Sized {
        true
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub enum MessageDestination {
    All,
    Single(NodeId),
    Multiple(Vec<NodeId>),
}

impl MessageDestination {
    pub fn is_included(&self, node_id: NodeId) -> bool {
        match self {
            MessageDestination::All => true,
            MessageDestination::Single(id) => *id == node_id,
            MessageDestination::Multiple(ids) => ids.contains(&node_id),
        }
    }
}
