use crate::routers::source_spray_and_wait::message::Message;
use crate::utils::{NodeId, Data, MessageId};
use serde::{Deserialize, Serialize};
use crate::binary_serde::BinarySerde;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Clone)]
pub enum Packet {
    Ping {
        source_id: NodeId,
    },
    Fulfillment {
        source_id: NodeId,
        destination_id: NodeId,
        message_data: Vec<MessageData>,
    },
    Acknowledgement {
        source_id: NodeId,
        destination_id: NodeId,
        message_ids: Vec<MessageId>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageData {
    pub remaining_copies: u32,
    pub remaining_destinations: Option<HashSet<NodeId>>,
    pub message: Data,
}

impl BinarySerde for Packet {}
impl BinarySerde for MessageData {}