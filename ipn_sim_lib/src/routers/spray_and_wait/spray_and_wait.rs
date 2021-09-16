use crate::router::Router;
use crate::utils::{NodeId, Data, TimeMetric, MessageId};
use crate::routers::spray_and_wait::flavour::Flavour;
use crate::router_link::RouterLink;
use crate::events::create_message_event::MessageDestination;
use bit_vec::BitBlock;
use crate::routers::spray_and_wait::message::Message;
use crate::message_buffer::MessageHandle;
use crate::binary_serde::BinarySerde;

#[derive(Clone)]
pub struct SprayAndWait<F: Flavour> {
    pub node_id: Option<NodeId>,
    next_message_index: u16,
    messages_to_forward: Vec<MessageToForward>,
    flavour: F
}

pub struct MessageToForward {
    handle: MessageHandle,
    remaining_destinations: MessageDestination,
    remaining_copies: u32,
}

impl<F: Flavour> SprayAndWait<F> {
    pub fn new() -> Self {
        Self {
            node_id: None,
            next_message_index: 0,
            messages_to_forward: vec![],
            flavour: F::new(),
        }
    }
}

impl<F: Flavour> Router for SprayAndWait<F> {
    fn on_init(&mut self, _link: &mut RouterLink, id: NodeId) {
        self.node_id = Some(id);
    }

    fn on_message_created(&mut self, link: &mut RouterLink, destination: MessageDestination, payload: Data, ttl: Option<TimeMetric>) {
        let message_id = (self.node_id.unwrap() as MessageId)
            .overflowing_shl((NodeId::bytes() as u32) * 8)
            .0 | self.next_message_index as MessageId;
        self.next_message_index += 1;
        link.report_message_created(message_id, destination.clone(), ttl);
        let message = Message {
            id: message_id,
            source_id: self.node_id.unwrap(),
            destination,
            payload,
            ttl,
        };
        if let Some(message_handle) = link.add_to_message_buffer(message.as_data()) {
            self.messages_to_forward.push(MessageToForward {
                handle: message_handle,
                remaining_destinations: destination.clone(),
                remaining_copies: 50
            })
        }
    }

    // fn on_data_received(&mut self, link: &mut RouterLink, data: Data) {
    //     todo!()
    // }
}