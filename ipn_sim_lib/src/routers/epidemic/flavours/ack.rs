use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

use crate::binary_serde::BinarySerde;
use crate::router_link::RouterLink;
use crate::routers::epidemic::epidemic::Epidemic;
use crate::routers::epidemic::flavour::Flavour;
use crate::routers::epidemic::message::Message;
use crate::utils::NodeId;

#[derive(Clone)]
pub struct Ack {
    ack_vector: BitVec,
}

#[derive(Serialize, Deserialize, Hash)]
pub struct PingPacket {
    source_id: NodeId,
    summary_vector: BitVec,
    ack_vector: BitVec,
}

#[derive(Serialize, Deserialize, Hash)]
pub struct RequestPacket {
    source_id: NodeId,
    destination_id: NodeId,
    request_vector: BitVec,
    ack_vector: BitVec,
}

#[derive(Serialize, Deserialize, Hash)]
pub struct FulfillmentPacket {
    source_id: NodeId,
    destination_id: NodeId,
    messages: Vec<Message>,
}

impl Ack {
    fn update_ack_vector(
        &mut self,
        router: &mut Epidemic<Self>,
        link: &mut RouterLink,
        other_ack_vector: &BitVec,
    ) {
        if other_ack_vector.any() {
            let acked_message_id_hashes = router
                .message_table
                .keys()
                .filter(|message_id_hash| other_ack_vector.get(**message_id_hash).unwrap())
                .copied()
                .collect::<Vec<_>>();

            for message_id_hash in acked_message_id_hashes {
                if other_ack_vector.get(message_id_hash).unwrap() {
                    if let Some(message_handle) = router.message_table.remove(&message_id_hash) {
                        link.remove_from_message_buffer(message_handle);
                        self.ack_vector.set(message_id_hash, true);
                    }
                }
            }
            // other_ack_vector
            //     .iter()
            //     .enumerate()
            //     .for_each(|(message_id_hash, acked)| {
            //         if acked {
            //             if let Some(message_handle) = router.message_table.remove(&message_id_hash) {
            //                 link.remove_from_message_buffer(message_handle);
            //                 self.ack_vector.set(message_id_hash, true);
            //             }
            //         }
            //     })
        }
    }
}

impl Flavour for Ack {
    type PingPacket = PingPacket;
    type RequestPacket = RequestPacket;
    type FulfillmentPacket = FulfillmentPacket;

    fn new(vector_size: usize) -> Self {
        Self {
            ack_vector: BitVec::from_elem(vector_size, false),
        }
    }

    fn make_ping_packet(&self, router: &Epidemic<Self>) -> Self::PingPacket {
        Self::PingPacket {
            source_id: router.node_id.unwrap(),
            summary_vector: router.summary_vector.clone(),
            ack_vector: self.ack_vector.clone(),
        }
    }

    fn make_request_packet(
        &self,
        source_id: NodeId,
        destination_id: NodeId,
        request_vector: BitVec,
    ) -> Self::RequestPacket {
        Self::RequestPacket {
            source_id,
            destination_id,
            request_vector,
            ack_vector: self.ack_vector.clone(),
        }
    }

    fn make_fulfillment_packet(
        &self,
        source_id: NodeId,
        destination_id: NodeId,
        messages: Vec<Message>,
    ) -> Self::FulfillmentPacket {
        Self::FulfillmentPacket {
            source_id,
            destination_id,
            messages,
        }
    }

    fn handle_ping_packet(
        &mut self,
        router: &mut Epidemic<Self>,
        link: &mut RouterLink,
        packet: Self::PingPacket,
    ) {
        self.update_ack_vector(router, link, &packet.ack_vector);
        router.handle_ping_packet(link, self, packet.source_id, packet.summary_vector);
    }

    fn handle_request_packet(
        &mut self,
        router: &mut Epidemic<Self>,
        link: &mut RouterLink,
        packet: Self::RequestPacket,
    ) {
        self.update_ack_vector(router, link, &packet.ack_vector);
        router.handle_request_packet(
            link,
            self,
            packet.source_id,
            packet.destination_id,
            &packet.request_vector,
        );
    }

    fn handle_fulfillment_packet(
        &mut self,
        router: &mut Epidemic<Self>,
        link: &mut RouterLink,
        packet: Self::FulfillmentPacket,
    ) {
        router.handle_fulfillment_packet(
            link,
            self,
            packet.source_id,
            packet.destination_id,
            &packet.messages,
        )
    }

    fn on_ttl_evict(&mut self, message_id_hash: usize) {
        // self.ack_vector.set(message_id_hash, false)
    }

    fn on_message_delivered(&mut self, message_id_hash: usize) {
        self.ack_vector.set(message_id_hash, true);
    }
}

impl BinarySerde for PingPacket {}
impl BinarySerde for RequestPacket {}
impl BinarySerde for FulfillmentPacket {}
