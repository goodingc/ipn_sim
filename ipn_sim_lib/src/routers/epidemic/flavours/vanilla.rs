use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

use crate::binary_serde::BinarySerde;
use crate::router_link::RouterLink;
use crate::routers::epidemic::epidemic::Epidemic;
use crate::routers::epidemic::flavour::Flavour;
use crate::routers::epidemic::message::Message;
use crate::utils::NodeId;

#[derive(Clone)]
pub struct Vanilla;

#[derive(Serialize, Deserialize)]
pub struct PingPacket {
    source_id: NodeId,
    summary_vector: BitVec,
}

#[derive(Serialize, Deserialize)]
pub struct RequestPacket {
    source_id: NodeId,
    destination_id: NodeId,
    request_vector: BitVec,
}

#[derive(Serialize, Deserialize)]
pub struct FulfillmentPacket {
    source_id: NodeId,
    destination_id: NodeId,
    messages: Vec<Message>,
}

impl Flavour for Vanilla {
    type PingPacket = PingPacket;
    type RequestPacket = RequestPacket;
    type FulfillmentPacket = FulfillmentPacket;

    fn new(_vector_size: usize) -> Self {
        Self
    }

    fn make_ping_packet(&self, router: &Epidemic<Self>) -> Self::PingPacket {
        Self::PingPacket {
            source_id: router.node_id.unwrap(),
            summary_vector: router.summary_vector.clone(),
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
        router.handle_ping_packet(link, self, packet.source_id, packet.summary_vector);
    }

    fn handle_request_packet(
        &mut self,
        router: &mut Epidemic<Self>,
        link: &mut RouterLink,
        packet: Self::RequestPacket,
    ) {
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
}

impl BinarySerde for PingPacket {}

impl BinarySerde for RequestPacket {}

impl BinarySerde for FulfillmentPacket {}
