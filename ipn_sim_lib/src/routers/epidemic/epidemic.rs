use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;

use bit_vec::{BitBlock, BitVec};
use rand::random;

use crate::binary_serde::BinarySerde;
use crate::node::message_buffer::MessageHandle;
use crate::router::Router;
use crate::router_link::RouterLink;
use crate::routers::epidemic::flavour::Flavour;
use crate::routers::epidemic::message::Message;
use crate::routers::epidemic::packet::Packet;
use crate::utils::{Data, MessageId, NodeId, TimeMetric};

use rustc_hash::{FxHashMap, FxHasher};
use crate::message_destination::{MessageDestination, IsIncluded};

#[derive(Clone)]
pub struct Epidemic<F: Flavour> {
    pub node_id: Option<NodeId>,
    pub summary_vector: BitVec,
    pub message_table: FxHashMap<usize, MessageHandle>,
    last_ping: FxHashMap<NodeId, TimeMetric>,
    reconnect_time: TimeMetric,
    next_message_index: u16,
    flavour: Option<F>,
}

impl<F: Flavour> Epidemic<F> {
    pub fn new(summary_vector_size: usize, reconnect_time: TimeMetric) -> Self {
        Self {
            node_id: None,
            summary_vector: BitVec::from_elem(summary_vector_size, false),
            message_table: FxHashMap::default(),
            last_ping: FxHashMap::default(),
            reconnect_time,
            next_message_index: 0,
            flavour: Some(F::new(summary_vector_size)),
        }
    }

    fn ping(&mut self, link: &mut RouterLink) {
        link.add_to_transmit_buffer(
            Packet::<F::PingPacket, F::RequestPacket, F::FulfillmentPacket>::Ping(
                self.flavour.as_ref().unwrap().make_ping_packet(self),
            )
                .as_data(),
        );
        link.sleep_for(1_000_000_000 * 60 * 15);
    }

    pub fn handle_ping_packet(
        &mut self,
        link: &mut RouterLink,
        flavour: &F,
        source_id: NodeId,
        mut summary_vector: BitVec,
    ) {
        if summary_vector.any() {
            let should_reply = self
                .last_ping
                .get(&source_id)
                .map_or(true, |last_fulfillment| {
                    link.get_time() - last_fulfillment > self.reconnect_time
                });
            if should_reply {
                let mut request_vector = self.summary_vector.clone();
                request_vector.negate();
                request_vector.and(&summary_vector);
                // summary_vector.and(&request_vector);
                link.add_to_transmit_buffer(
                    Packet::<F::PingPacket, F::RequestPacket, F::FulfillmentPacket>::Request(
                        flavour.make_request_packet(
                            self.node_id.unwrap(),
                            source_id,
                            request_vector,
                        ),
                    )
                        .as_data(),
                );
            }
        }
    }

    pub fn handle_request_packet(
        &mut self,
        link: &mut RouterLink,
        flavour: &mut F,
        source_id: NodeId,
        destination_id: NodeId,
        request_vector: &BitVec,
    ) {
        if destination_id == self.node_id.unwrap() {
            let requested_message_id_hashes = self
                .message_table
                .keys()
                .filter(|&message_id_hash| request_vector.get(*message_id_hash).unwrap())
                .copied()
                .collect::<Vec<_>>();

            let messages = requested_message_id_hashes
                .iter()
                .filter_map(|message_id_hash| {
                    if let Some(&message_handle) = self.message_table.get(message_id_hash) {
                        if self.verify_ttl(link, &message_handle) {
                            flavour.on_ttl_evict(*message_id_hash)
                        }
                        if let Some(message_data) = link.get_from_message_buffer(&message_handle) {
                            let message = Message::from_data(message_data);
                            link.report_message_sent(message.id, source_id);
                            return Some(message);
                        }
                    }
                    None
                })
                .collect::<Vec<_>>();

            if messages.len() > 0 {
                link.add_to_transmit_buffer(
                    Packet::<F::PingPacket, F::RequestPacket, F::FulfillmentPacket>::Fulfillment(
                        flavour.make_fulfillment_packet(self.node_id.unwrap(), source_id, messages),
                    )
                        .as_data(),
                );
            }
        }
    }

    pub fn handle_fulfillment_packet(
        &mut self,
        link: &mut RouterLink,
        flavour: &mut F,
        source_id: NodeId,
        destination_id: NodeId,
        messages: &Vec<Message>,
    ) {
        if destination_id == self.node_id.unwrap() {
            for message in messages {
                link.report_message_received(message.id, source_id);
                self.insert_message(link, flavour, message.id, message.as_data())
            }
        }
    }

    fn get_message_id_hash(&self, message_id: MessageId) -> usize {
        let mut hasher = DefaultHasher::default();
        hasher.write(message_id.to_le_bytes().borrow());
        hasher.finish() as usize % self.summary_vector.len()
        // message_id as usize % self.summary_vector.len()
    }

    fn insert_message(
        &mut self,
        link: &mut RouterLink,
        flavour: &mut F,
        id: MessageId,
        data: Data,
    ) {
        let hash = self.get_message_id_hash(id);
        let message = Message::from_data(&data);
        if !self.summary_vector.get(hash).unwrap() {
            let at_destination = message.destination.is_included(&self.node_id.unwrap());
            if at_destination {
                link.report_message_delivered(message.id, message.source_id);
                flavour.on_message_delivered(hash);
            }
            self.summary_vector.set(hash, true);
            if let Some(message_handle) = link.add_to_message_buffer(data) {
                self.message_table.insert(hash, message_handle);
            } else {
                link.report_message_dropped(id);
            }
        } else {
            link.report_message_dropped(id);
        }
    }

    fn verify_ttl(&mut self, link: &mut RouterLink, message_handle: &MessageHandle) -> bool {
        if let Some(message_data) = link.get_from_message_buffer(message_handle) {
            let message = Message::from_data(message_data);
            if let Some(ttl) = message.ttl {
                if ttl <= link.get_time() {
                    link.remove_from_message_buffer(&message_handle);
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

impl<F: Flavour> Router for Epidemic<F> {
    fn on_init(&mut self, link: &mut RouterLink, id: NodeId) {
        self.node_id = Some(id);
        link.sleep_for(random::<TimeMetric>() % (1_000_000_000 * 60 * 15))
    }

    fn on_message_created(
        &mut self,
        link: &mut RouterLink,
        destination: MessageDestination<NodeId>,
        payload: Data,
        ttl: Option<TimeMetric>,
    ) {
        let message_id = (self.node_id.unwrap() as MessageId)
            .overflowing_shl((NodeId::bytes() as u32) * 8)
            .0
            | self.next_message_index as MessageId;
        self.next_message_index += 1;
        link.report_message_created(message_id, destination.clone(), ttl);
        let message = Message {
            id: message_id,
            source_id: self.node_id.unwrap(),
            destination,
            payload,
            ttl,
        };
        let mut flavour = self.flavour.take().unwrap();
        self.insert_message(link, &mut flavour, message_id, message.as_data());
        self.flavour = Some(flavour);
    }

    fn on_data_received(&mut self, link: &mut RouterLink, data: Data) {
        let mut flavour = self.flavour.take().unwrap();
        match Packet::from_data(&data) {
            Packet::Ping(packet) => {
                flavour.handle_ping_packet(self, link, packet);
            }
            Packet::Request(packet) => flavour.handle_request_packet(self, link, packet),
            Packet::Fulfillment(packet) => flavour.handle_fulfillment_packet(self, link, packet),
        }
        self.flavour = Some(flavour);
    }

    fn on_awake(&mut self, link: &mut RouterLink) {
        let handles_to_verify = self.message_table.values().copied().collect::<Vec<_>>();
        for message_handle in handles_to_verify {
            self.verify_ttl(link, &message_handle);
        }
        self.ping(link);
    }
}
