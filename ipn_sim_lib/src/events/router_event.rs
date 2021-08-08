use crate::event::Event;
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::node::node::Node;
use crate::utils;
use crate::utils::{Data, MessageId, NodeId, TimeMetric};
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::Shared;


#[derive(Clone)]
pub struct RouterEvent {
    pub node: Shared<Node>,
    pub event_type: RouterEventType,
}

#[derive(Clone)]
pub enum RouterEventType {
    Log(String),
    MessageCreated {
        id: MessageId,
        destination: MessageDestination,
        ttl: Option<TimeMetric>,
    },
    MessageSent {
        id: MessageId,
        destination_node: Shared<Node>,
    },
    MessageReceived {
        id: MessageId,
        source_node: Shared<Node>,
    },
    MessageDropped {
        id: MessageId,
    },
    MessageDelivered {
        id: MessageId,
        source_node: Shared<Node>,
    },
}

#[derive(Clone)]
pub enum MessageDestination {
    All,
    Single(Shared<Node>),
    Multiple(Vec<Shared<Node>>),
}

impl RouterEvent {
    pub fn new(node: &Shared<Node>, event_type: RouterEventType) -> Self {
        Self {
            node: Rc::clone(node),
            event_type,
        }
    }
}

impl Event for RouterEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        if let RouterEventType::Log(message) = self.event_type {
            utils::log(&*format!(
                "[{}] [{}] {}",
                sim.time,
                self.node.borrow().name,
                message
            ));
        }
    }

    fn is_internal() -> bool where Self: Sized {
        true
    }
}
