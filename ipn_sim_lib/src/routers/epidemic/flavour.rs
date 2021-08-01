use bit_vec::BitVec;

use crate::binary_serde::BinarySerde;
use crate::router_link::RouterLink;
use crate::routers::epidemic::epidemic::Epidemic;
use crate::routers::epidemic::message::Message;
use crate::utils::NodeId;


pub trait Flavour: Clone {
    type PingPacket: BinarySerde;
    type RequestPacket: BinarySerde;
    type FulfillmentPacket: BinarySerde;

    fn new(vector_size: usize) -> Self;

    fn make_ping_packet(&self, router: &Epidemic<Self>) -> Self::PingPacket
        where
            Self: Sized;

    fn make_request_packet(
        &self,
        source_id: NodeId,
        destination_id: NodeId,
        request_vector: BitVec,
    ) -> Self::RequestPacket;

    fn make_fulfillment_packet(
        &self,
        source_id: NodeId,
        destination_id: NodeId,
        messages: Vec<Message>,
    ) -> Self::FulfillmentPacket;

    fn handle_ping_packet(
        &mut self,
        router: &mut Epidemic<Self>,
        link: &mut RouterLink,
        packet: Self::PingPacket,
    ) where
        Self: Sized;

    fn handle_request_packet(
        &mut self,
        router: &mut Epidemic<Self>,
        link: &mut RouterLink,
        packet: Self::RequestPacket,
    ) where
        Self: Sized;

    fn handle_fulfillment_packet(
        &mut self,
        router: &mut Epidemic<Self>,
        link: &mut RouterLink,
        packet: Self::FulfillmentPacket,
    ) where
        Self: Sized;

    fn on_ttl_evict(&mut self, _message_id_hash: usize) {}

    fn on_message_delivered(&mut self, _message_id_hash: usize) {}
}
