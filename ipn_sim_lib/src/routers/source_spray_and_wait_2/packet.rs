use crate::utils::NodeId;
use bit_vec::BitVec;
use crate::routers::source_spray_and_wait_2::message::Message;
use crate::binary_serde::BinarySerde;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub enum Packet {
    Ping {
        source_id: NodeId,
        summary_vector: BitVec,
        ack_vector: BitVec,
    },
    Request {
        source_id: NodeId,
        destination_id: NodeId,
        request_vector: BitVec,
        ack_vector: BitVec,
    },
    Fulfillment {
        source_id: NodeId,
        destination_id: NodeId,
        messages: Vec<(Message, u16)>
    }
}

impl BinarySerde for Packet {}