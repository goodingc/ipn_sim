use crate::events::awake_router_event::AwakeRouterEvent;
use crate::events::create_message_event;
use crate::events::router_event::{MessageDestination, RouterEvent, RouterEventType};
use crate::events::transmit_start_event::TransmitStartEvent;
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::message_buffer::MessageHandle;
use crate::node::node::Node;
use crate::utils::{Data, MessageId, NodeId, TimeMetric};
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::Shared;


pub struct RouterLink<'a> {
    node: &'a mut Node,
    node_rc: Shared<Node>,
    sim: &'a mut IpnSim,
}

impl<'a> RouterLink<'a> {
    pub fn new(node: &'a mut Node, node_rc: &Shared<Node>, sim: &'a mut IpnSim) -> Self {
        Self {
            node,
            node_rc: Rc::clone(node_rc),
            sim,
        }
    }

    pub fn add_to_message_buffer(&mut self, data: Data) -> Option<MessageHandle> {
        self.node.message_buffer.add_message(data)
    }

    pub fn get_from_message_buffer(&self, message_handle: MessageHandle) -> Option<&Data> {
        self.node.message_buffer.get_message(message_handle)
    }

    pub fn clone_from_message_buffer(&mut self, message_handle: MessageHandle) -> Option<Data> {
        self.node
            .message_buffer
            .get_message(message_handle)
            .cloned()
    }

    pub fn remove_from_message_buffer(&mut self, message_handle: MessageHandle) -> Option<Data> {
        self.node.message_buffer.remove_message(message_handle)
    }

    pub fn add_to_transmit_buffer(&mut self, data: Data) {
        let transmit_start = self.node.transceiver.add_to_buffer(data, self.sim.time);
        self.sim.add_event(
            transmit_start,
            TransmitStartEvent {
                node: Rc::clone(&self.node_rc),
            },
        );
    }

    pub fn sleep_for(&mut self, sleep_time: TimeMetric) {
        self.sim.add_event(
            self.sim.time + sleep_time,
            AwakeRouterEvent {
                node: Rc::clone(&self.node_rc),
            },
        );
    }

    pub fn get_time(&self) -> TimeMetric {
        self.sim.time
    }

    pub fn get_single_message_destination(&self, node_id: NodeId) -> MessageDestination {
        MessageDestination::Single(self.sim.get_node(node_id))
    }

    pub fn get_multiple_message_destination(&self, node_ids: &Vec<NodeId>) -> MessageDestination {
        MessageDestination::Multiple(node_ids.iter().map(|id| self.sim.get_node(*id)).collect())
    }

    fn report(&mut self, event_type: RouterEventType) {
        self.sim
            .add_event(self.sim.time, RouterEvent::new(&self.node_rc, event_type));
    }

    pub fn log(&mut self, message: impl Into<String>) {
        self.report(RouterEventType::Log(message.into()));
    }

    pub fn report_message_created(
        &mut self,
        id: MessageId,
        destination: create_message_event::MessageDestination,
        ttl: Option<TimeMetric>,
    ) {
        self.report(RouterEventType::MessageCreated {
            id: id,
            destination: match destination {
                create_message_event::MessageDestination::All => MessageDestination::All,
                create_message_event::MessageDestination::Single(id) => {
                    self.get_single_message_destination(id)
                }
                create_message_event::MessageDestination::Multiple(ids) => {
                    self.get_multiple_message_destination(&ids)
                }
            },
            ttl,
        });
    }

    pub fn report_message_sent(&mut self, message_id: MessageId, destination_node_id: NodeId) {
        self.report(RouterEventType::MessageSent {
            id: message_id,
            destination_node: self.sim.get_node(destination_node_id),
        })
    }

    pub fn report_message_received(&mut self, message_id: MessageId, source_node_id: NodeId) {
        self.report(RouterEventType::MessageReceived {
            id: message_id,
            source_node: self.sim.get_node(source_node_id),
        })
    }

    pub fn report_message_delivered(&mut self, message_id: MessageId, source_node_id: NodeId) {
        self.report(RouterEventType::MessageDelivered {
            id: message_id,
            source_node: self.sim.get_node(source_node_id),
        })
    }
}
