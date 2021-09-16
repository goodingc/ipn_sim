use std::collections::{HashMap, HashSet};
use std::iter;

use bit_vec::BitBlock;
use rand::random;

use crate::binary_serde::BinarySerde;
use crate::message_destination::{MessageDestination, IsIncluded};
use crate::node::message_buffer::MessageHandle;
use crate::router::Router;
use crate::router_link::RouterLink;
use crate::routers::source_spray_and_wait::message::Message;
use crate::routers::source_spray_and_wait::packet::{MessageData, Packet};
use crate::utils::{Data, log, MessageId, NodeId, TimeMetric};

#[derive(Clone)]
pub struct SourceSprayAndWait {
    next_message_index: u16,
    messages_to_forward: HashMap<MessageId, MessageToForward>,
}

#[derive(Clone)]
pub struct MessageToForward {
    handle: MessageHandle,
    remaining_destinations: Option<HashSet<NodeId>>,
    remaining_copies: u32,
}

impl SourceSprayAndWait {
    pub fn new() -> Self {
        Self {
            next_message_index: 0,
            messages_to_forward: HashMap::new(),
        }
    }
}

impl SourceSprayAndWait {
    fn ping(&mut self, link: &mut RouterLink) {
        link.add_to_transmit_buffer(
            Packet::Ping {
                source_id: link.get_node_id()
            }.as_data()
        );
        link.sleep_for(1_000_000_000 * 60 * 15);
    }
}

impl Router for SourceSprayAndWait {
    fn on_init(&mut self, link: &mut RouterLink, _id: NodeId) {
        link.sleep_for(random::<TimeMetric>() % (1_000_000_000 * 60 * 15))
    }

    fn on_message_created(&mut self, link: &mut RouterLink, destination: MessageDestination<NodeId>, payload: Data, ttl: Option<TimeMetric>) {
        let message_id = (link.get_node_id() as MessageId)
            .overflowing_shl((NodeId::bytes() as u32) * 8)
            .0 | self.next_message_index as MessageId;
        self.next_message_index += 1;

        link.report_message_created(message_id, destination.clone(), ttl);

        let message = Message {
            id: message_id,
            source_id: link.get_node_id(),
            destination: destination.clone(),
            payload,
            ttl,
        };

        if let Some(message_handle) = link.add_to_message_buffer(message.as_data()) {
            self.messages_to_forward.insert(message_id, MessageToForward {
                handle: message_handle,
                remaining_destinations: match destination {
                    MessageDestination::All => None,
                    MessageDestination::Single(id) => Some(iter::once(id).collect()),
                    MessageDestination::Multiple(ids) => Some(ids
                        .into_iter()
                        .collect())
                },
                remaining_copies: 50,
            });
        }
    }

    fn on_data_received(&mut self, link: &mut RouterLink, data: Data) {
        let packet = Packet::from_data(&data);
        match packet {
            Packet::Ping { source_id } => {
                let message_data = self.messages_to_forward
                    .iter_mut()
                    .filter(|(_, message_to_forward)|
                        message_to_forward.remaining_copies > 1 ||
                            message_to_forward.remaining_destinations
                                .as_ref()
                                .map_or(
                                    true,
                                    |destinations| destinations.contains(&source_id),
                                )
                    ).map(|(message_id, message_to_forward)| {
                    message_to_forward.remaining_copies -= 1;

                    link.report_message_sent(*message_id, source_id);

                    MessageData {
                        remaining_copies: message_to_forward.remaining_copies,
                        remaining_destinations: message_to_forward.remaining_destinations.clone(),
                        message: link.clone_from_message_buffer(&message_to_forward.handle).unwrap(),
                    }
                }).collect();
                link.add_to_transmit_buffer(Packet::Fulfillment {
                    source_id: link.get_node_id(),
                    destination_id: source_id,
                    message_data,
                }.as_data())
            }
            Packet::Fulfillment {
                source_id,
                destination_id,
                message_data
            } => {
                if destination_id == link.get_node_id() {
                    let ack_message_ids = message_data
                        .into_iter()
                        .filter_map(|mut message_data| {
                            let message = Message::from_data(&message_data.message);

                            link.report_message_received(message.id, source_id);

                            if self.messages_to_forward.contains_key(&message.id) {
                                Some(message.id)
                            } else {
                                link.add_to_message_buffer(message_data.message.clone())
                                    .map(|message_handle| {
                                        if message.destination.is_included(&destination_id) {
                                            link.log(format!(
                                                "delivered, id: {}, mtf_len: {}, d_id: {}, n_id: {}, mtf_p: {:p}",
                                                message.id,
                                                self.messages_to_forward.len(),
                                                destination_id,
                                                link.get_node_id(),
                                                &self.messages_to_forward
                                            ));
                                            // link.report_message_delivered(message.id, message.source_id);
                                        }

                                        if let Some(remaining_destinations) = &mut message_data.remaining_destinations {
                                            remaining_destinations.remove(&link.get_node_id());
                                        }

                                        self.messages_to_forward.insert(message.id, MessageToForward {
                                            handle: message_handle,
                                            remaining_destinations: message_data.remaining_destinations,
                                            remaining_copies: message_data.remaining_copies - 1,
                                        });

                                        if message.destination.is_included(&destination_id) {
                                            link.log(format!(
                                                "delivered, id: {}, mtf_len: {}, d_id: {}, n_id: {}, mtf_p: {:p}",
                                                message.id,
                                                self.messages_to_forward.len(),
                                                destination_id,
                                                link.get_node_id(),
                                                &self.messages_to_forward
                                            ));
                                            link.report_message_delivered(message.id, message.source_id);
                                        }

                                        message.id
                                    })
                            }
                        }).collect();

                    link.add_to_transmit_buffer(Packet::Acknowledgement {
                        source_id: destination_id,
                        destination_id: source_id,
                        message_ids: ack_message_ids,
                    }.as_data())
                }
            }
            Packet::Acknowledgement {
                source_id,
                destination_id,
                message_ids
            } => {
                if destination_id == link.get_node_id() {
                    for message_id in message_ids {
                        let mut should_remove = false;
                        if let Some(message_to_forward) = self.messages_to_forward.get_mut(&message_id) {
                            if let Some(remaining_destinations) = &mut message_to_forward.remaining_destinations {
                                remaining_destinations.remove(&source_id);
                                if remaining_destinations.is_empty() {
                                    should_remove = true;
                                }
                            }
                        }

                        if should_remove {
                            // self.messages_to_forward.remove(&message_id);
                        }
                    }
                }
            }
        }
    }

    fn on_awake(&mut self, link: &mut RouterLink) {
        self.ping(link);
    }
}