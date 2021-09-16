use bit_vec::{BitVec, BitBlock};
use rustc_hash::FxHashMap;

use crate::node::message_buffer::MessageHandle;
use crate::utils::{NodeId, TimeMetric, Data, MessageId};
use crate::router::Router;
use crate::router_link::RouterLink;
use rand::random;
use crate::routers::epidemic_2::message::Message;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use crate::binary_serde::BinarySerde;
use crate::routers::epidemic_2::packet::Packet;
use crate::message_destination::{MessageDestination, IsIncluded};

#[derive(Clone)]
pub struct Epidemic {
    summary_vector: BitVec,
    ack_vector: Option<BitVec>,
    message_table: FxHashMap<usize, MessageHandle>,
    message_meta: FxHashMap<MessageHandle, (MessageId, Option<TimeMetric>, usize)>,
    last_ping_time: Option<TimeMetric>,
    next_message_index: u16,
    reconnect_time: TimeMetric,
    ping_on_receive: bool,
    bilateral_ack: bool,
    bilateral_fulfillment: bool,
}

#[derive(Eq, PartialEq)]
pub enum Ack {
    None,
    Unilateral,
    Bilateral
}

impl Epidemic {
    pub fn new(summary_vector_size: usize, reconnect_time: TimeMetric, ping_on_receive: bool, ack: Ack, bilateral_fulfillment: bool) -> Self {
        Self {
            summary_vector: BitVec::from_elem(summary_vector_size, false),
            ack_vector: (ack != Ack::None).then(|| BitVec::from_elem(summary_vector_size, false)),
            message_table: FxHashMap::default(),
            message_meta: FxHashMap::default(),
            last_ping_time: None,
            next_message_index: 0,
            reconnect_time,
            ping_on_receive,
            bilateral_ack: (ack == Ack::Bilateral),
            bilateral_fulfillment,
        }
    }

    fn insert_message(
        &mut self,
        link: &mut RouterLink,
        message: Message,
    ) {
        let hash = self.get_message_id_hash(message.id);

        if !self.summary_vector.get(hash).unwrap() {
            let at_destination = message.destination.is_included(&link.get_node_id());
            if at_destination {
                link.report_message_delivered(message.id, message.source_id);
                self.summary_vector.set(hash, true);

                if let Some(ack_vector) = &mut self.ack_vector {
                    ack_vector.set(hash, true);
                }

                link.report_message_dropped(message.id);
            } else {
                if let Some(message_handle) = link.add_to_message_buffer(message.as_data()) {
                    self.message_table.insert(hash, message_handle);
                    self.message_meta.insert(message_handle, (message.id, message.ttl, hash));
                    self.summary_vector.set(hash, true);
                } else {
                    link.report_message_dropped(message.id);
                }
            }
        } else {
            link.report_message_dropped(message.id);
        }
    }

    fn get_message_id_hash(&self, message_id: MessageId) -> usize {
        let mut hasher = DefaultHasher::default();
        hasher.write(&message_id.to_le_bytes());
        hasher.finish() as usize % self.summary_vector.len()
    }

    fn verify_ttl(&mut self, link: &mut RouterLink, message_handle: &MessageHandle) {
        let mut evict = false;
        if let Some(ttl) = self.message_meta.get(message_handle).unwrap().1 {
            if ttl <= link.get_time() {
                evict = true;
            }
        }

        if evict {
            let (id, _ttl, id_hash) = self.message_meta.remove(message_handle).unwrap();

            link.remove_from_message_buffer(message_handle);
            self.message_table.remove(&id_hash);
            self.summary_vector.set(id_hash, false);

            link.report_message_dropped(id);
        }
    }

    fn try_ping(&mut self, link: &mut RouterLink, force: bool) {
        let should_ping = self.last_ping_time.map_or(
            true,
            |last_ping_time| last_ping_time + self.reconnect_time <= link.get_time(),
        );

        if should_ping || force {
            link.add_to_transmit_buffer(Packet::Ping {
                source_id: link.get_node_id(),
                summary_vector: self.summary_vector.clone(),
                ack_vector: self.ack_vector.clone(),
            }.as_data());

            self.last_ping_time = Some(link.get_time());
            link.sleep_for(self.reconnect_time);
        }
    }

    fn handle_ack_vector(&mut self, link: &mut RouterLink, other_ack_vector: &Option<BitVec>) {
        if let Some(other_ack_vector) = other_ack_vector {
            if other_ack_vector.any() {
                if let Some(ack_vector) = &mut self.ack_vector {
                    let acked_message_id_hashes = self.message_table
                        .keys()
                        .filter(|&message_id_hash| other_ack_vector.get(*message_id_hash).unwrap())
                        .copied()
                        .collect::<Vec<_>>();

                    for message_id_hash in acked_message_id_hashes {
                        let message_handle = self.message_table.remove(&message_id_hash).unwrap();

                        link.remove_from_message_buffer(&message_handle);
                        let (id, ..) = self.message_meta.remove(&message_handle).unwrap();
                        ack_vector.set(message_id_hash, true);

                        link.report_message_dropped(id);
                    }
                }
            }
        }
    }

    fn prepare_requested_messages(&mut self, link: &mut RouterLink, request_vector: &BitVec, source_id: NodeId) -> Vec<Message> {
        let requested_message_id_hashes = self
            .message_table
            .keys()
            .filter(|&message_id_hash| request_vector.get(*message_id_hash).unwrap())
            .copied()
            .collect::<Vec<_>>();

        requested_message_id_hashes
            .iter()
            .filter_map(|message_id_hash| {
                if let Some(&message_handle) = self.message_table.get(message_id_hash) {
                    self.verify_ttl(link, &message_handle);
                    if let Some(message_data) = link.get_from_message_buffer(&message_handle) {
                        let message = Message::from_data(message_data);
                        link.report_message_sent(message.id, source_id);
                        return Some(message);
                    }
                }
                None
            })
            .collect::<Vec<_>>()
    }

    fn handle_messages(&mut self, link: &mut RouterLink, messages: Vec<Message>, source_id: NodeId) {
        if !messages.is_empty() {
            for message in messages {
                link.report_message_received(message.id, source_id);
                self.insert_message(link, message)
            }
            if self.ping_on_receive {
                self.try_ping(link, true);
            }
        }
    }
}

impl Router for Epidemic {
    fn on_init(&mut self, link: &mut RouterLink, _id: NodeId) {
        link.sleep_for(random::<TimeMetric>() % self.reconnect_time)
    }

    fn on_message_created(&mut self, link: &mut RouterLink, destination: MessageDestination<NodeId>, payload: Data, ttl: Option<TimeMetric>) {
        let message_id = (link.get_node_id() as MessageId)
            .overflowing_shl((NodeId::bytes() as u32) * 8)
            .0 | self.next_message_index as MessageId;
        self.next_message_index += 1;

        link.report_message_created(message_id, destination.clone(), ttl);

        self.insert_message(link, Message {
            id: message_id,
            source_id: link.get_node_id(),
            destination,
            payload,
            ttl,
        });
    }

    fn on_data_received(&mut self, link: &mut RouterLink, data: Data) {
        match Packet::from_data(&data) {
            Packet::Ping {
                source_id, summary_vector, ack_vector
            } => {
                self.handle_ack_vector(link, &ack_vector);

                if summary_vector.any() {
                    let mut request_vector = self.summary_vector.clone();
                    request_vector.negate();
                    request_vector.and(&summary_vector);

                    if request_vector.any() {
                        link.add_to_transmit_buffer(Packet::Request {
                            source_id: link.get_node_id(),
                            destination_id: source_id,
                            request_vector,
                            ack_vector: if self.bilateral_ack {
                                self.ack_vector.clone()
                            } else {
                                None
                            },
                            summary_vector: if self.bilateral_fulfillment {
                                Some(self.summary_vector.clone())
                            } else {
                                None
                            },
                        }.as_data())
                    }
                }
            }
            Packet::Request {
                source_id,
                destination_id,
                request_vector,
                ack_vector,
                summary_vector
            } => {
                self.handle_ack_vector(link, &ack_vector);

                if destination_id == link.get_node_id() {
                    let messages = self.prepare_requested_messages(link, &request_vector, source_id);

                    if !messages.is_empty() || summary_vector.as_ref().map_or(false, BitVec::any) {
                        link.add_to_transmit_buffer(
                            Packet::Fulfillment {
                                source_id: link.get_node_id(),
                                destination_id: source_id,
                                messages,
                                request_vector: summary_vector.map(|summary_vector| {
                                    let mut request_vector = self.summary_vector.clone();
                                    request_vector.negate();
                                    request_vector.and(&summary_vector);
                                    request_vector
                                }),
                            }.as_data(),
                        );
                    }
                }
            }
            Packet::Fulfillment {
                source_id, destination_id, messages, request_vector
            } => {
                if destination_id == link.get_node_id() {
                    self.handle_messages(link, messages, source_id);

                    if let Some(request_vector) = request_vector {
                        if request_vector.any() {
                            let messages = self.prepare_requested_messages(link, &request_vector, source_id);

                            link.add_to_transmit_buffer(Packet::BilateralFulfillment {
                                source_id: destination_id,
                                destination_id: source_id,
                                messages,
                            }.as_data())
                        }
                    }
                }
            }
            Packet::BilateralFulfillment { source_id, destination_id, messages } => {
                if destination_id == link.get_node_id() {
                    self.handle_messages(link, messages, source_id);
                }
            }
        }
    }

    fn on_awake(&mut self, link: &mut RouterLink) {
        let handles_to_verify = self.message_table.values().copied().collect::<Vec<_>>();
        for handle in handles_to_verify {
            self.verify_ttl(link, &handle);
        }

        self.try_ping(link, false);
    }
}