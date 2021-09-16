use bit_vec::{BitVec, BitBlock};
use rustc_hash::FxHashMap;
use crate::node::message_buffer::MessageHandle;
use crate::router::Router;
use crate::router_link::RouterLink;
use crate::utils::{NodeId, Data, TimeMetric, MessageId};
use rand::random;
use crate::routers::source_spray_and_wait_2::packet::Packet;
use crate::binary_serde::BinarySerde;
use crate::routers::source_spray_and_wait_2::message::Message;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use crate::message_destination::{MessageDestination, IsIncluded};

#[derive(Clone)]
pub struct SourceSprayAndWait2 {
    summary_vector: BitVec,
    ack_vector: BitVec,
    message_table: FxHashMap<usize, (MessageHandle, u16)>,
    next_message_index: u16,
}

impl SourceSprayAndWait2 {
    pub fn new(summary_vector_size: usize) -> Self {
        Self {
            summary_vector: BitVec::from_elem(summary_vector_size, false),
            ack_vector: BitVec::from_elem(summary_vector_size, false),
            message_table: FxHashMap::default(),
            next_message_index: 0,
        }
    }

    fn update_ack_vector(
        &mut self,
        link: &mut RouterLink,
        other_ack_vector: &BitVec,
    ) {
        if other_ack_vector.any() {
            let acked_message_id_hashes = self.message_table
                .keys()
                .filter(|message_id_hash| other_ack_vector.get(**message_id_hash).unwrap())
                .copied()
                .collect::<Vec<_>>();

            for message_id_hash in acked_message_id_hashes {
                if let Some((message_handle, _)) = self.message_table.remove(&message_id_hash) {
                    link.remove_from_message_buffer(&message_handle);
                    self.ack_vector.set(message_id_hash, true);
                }
            }
        }
    }

    fn get_message_id_hash(&self, message_id: MessageId) -> usize {
        let mut hasher = DefaultHasher::default();
        hasher.write(&message_id.to_le_bytes());
        hasher.finish() as usize % self.summary_vector.len()
    }

    fn insert_message(
        &mut self,
        link: &mut RouterLink,
        message: Message,
        remaining_copies: u16,
    ) {
        let hash = self.get_message_id_hash(message.id);
        if !self.summary_vector.get(hash).unwrap() {
            let at_destination = message.destination.is_included(&link.get_node_id());
            if at_destination {
                link.report_message_delivered(message.id, message.source_id);
                self.ack_vector.set(hash, true);
            }
            self.summary_vector.set(hash, true);
            if let Some(message_handle) = link.add_to_message_buffer(message.as_data()) {
                self.message_table.insert(hash, (message_handle, remaining_copies));
            }
        }
    }

    fn ping(&mut self, link: &mut RouterLink) {
        link.add_to_transmit_buffer(
            Packet::Ping {
                source_id: link.get_node_id(),
                summary_vector: self.summary_vector.clone(),
                ack_vector: self.ack_vector.clone()
            }.as_data(),
        );
        link.sleep_for(1_000_000_000 * 60 * 15);
    }

    fn verify_ttl(&mut self, link: &mut RouterLink, message_handle: &MessageHandle) -> bool {
        if let Some(message_data) = link.get_from_message_buffer(message_handle) {
            let message = Message::from_data(message_data);
            if let Some(ttl) = message.ttl {
                if ttl <= link.get_time() {
                    link.remove_from_message_buffer(message_handle);
                    let message_id_hash = self.get_message_id_hash(message.id);
                    self.message_table.remove(&message_id_hash);
                    self.summary_vector.set(message_id_hash, false);
                    return true;
                }
            }
        }
        false
    }
}

impl Router for SourceSprayAndWait2 {
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
            destination,
            payload,
            ttl,
        };

        self.insert_message(link, message, 50);
    }

    fn on_data_received(&mut self, link: &mut RouterLink, data: Data) {
        let packet = Packet::from_data(&data);
        match packet {
            Packet::Ping {
                source_id,
                summary_vector,
                ack_vector
            } => {
                self.update_ack_vector(link, &ack_vector);
                if summary_vector.any() {
                    let mut request_vector = self.summary_vector.clone();
                    request_vector.negate();
                    request_vector.and(&summary_vector);
                    link.add_to_transmit_buffer(
                        Packet::Request {
                            source_id: link.get_node_id(),
                            destination_id: source_id,
                            request_vector,
                            ack_vector: self.ack_vector.clone()
                        }.as_data(),
                    );
                }
            }
            Packet::Request {
                source_id,
                destination_id,
                request_vector,
                ack_vector
            } => {
                self.update_ack_vector(link, &ack_vector);
                if destination_id == link.get_node_id() {
                    let requested_message_id_hashes = self
                        .message_table
                        .keys()
                        .filter(|&message_id_hash| request_vector.get(*message_id_hash).unwrap())
                        .copied()
                        .collect::<Vec<_>>();

                    let messages = requested_message_id_hashes
                        .iter()
                        .filter_map(|message_id_hash| {
                            if let Some(&(message_handle, _)) = self.message_table.get(message_id_hash) {
                                self.verify_ttl(link, &message_handle);
                            }
                            if let Some((message_handle, remaining_copies)) = self.message_table.get_mut(message_id_hash) {
                                if let Some(message_data) = link.get_from_message_buffer(message_handle) {
                                    let message = Message::from_data(message_data);

                                    if message.destination.is_included(&source_id) || *remaining_copies > 1 {
                                        link.report_message_sent(message.id, source_id);

                                        *remaining_copies /= 2;

                                        return Some((message, *remaining_copies));
                                    }
                                }
                            }
                            None
                        })
                        .collect::<Vec<_>>();

                    if messages.len() > 0 {
                        link.add_to_transmit_buffer(
                            Packet::Fulfillment {
                                source_id: destination_id,
                                destination_id: source_id,
                                messages,
                            }.as_data(),
                        );
                    }
                }
            }
            Packet::Fulfillment {
                source_id,
                destination_id,
                messages
            } => {
                if destination_id == link.get_node_id() {
                    for (message, remaining_copies) in messages {
                        link.report_message_received(message.id, source_id);
                        self.insert_message(link, message, remaining_copies);
                    }
                }
            }
        }
    }

    fn on_awake(&mut self, link: &mut RouterLink) {
        let handles_to_verify = self.message_table
            .values()
            .map(|&(message_handle, _)| message_handle)
            .collect::<Vec<_>>();
        for message_handle in handles_to_verify {
            self.verify_ttl(link, &message_handle);
        }
        self.ping(link);
    }
}