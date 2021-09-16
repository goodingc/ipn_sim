use crate::utils::NodeId;
use bit_vec::BitVec;
use crate::routers::epidemic_2::message::Message;
use crate::binary_serde::BinarySerde;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub enum Packet {
    Ping {
        source_id: NodeId,
        summary_vector: BitVec,
        ack_vector: Option<BitVec>,
    },
    Request {
        source_id: NodeId,
        destination_id: NodeId,
        request_vector: BitVec,
        ack_vector: Option<BitVec>,
        summary_vector: Option<BitVec>
    },
    Fulfillment {
        source_id: NodeId,
        destination_id: NodeId,
        messages: Vec<Message>,
        request_vector: Option<BitVec>
    },
    BilateralFulfillment {
        source_id: NodeId,
        destination_id: NodeId,
        messages: Vec<Message>,
    }
}

impl BinarySerde for Packet {}