use crate::utils::{Data, MessageId, NodeId, TimeMetric};

use crate::binary_serde::BinarySerde;
use serde::{Deserialize, Serialize};
use crate::message_destination::MessageDestination;

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: MessageId,
    pub source_id: NodeId,
    pub destination: MessageDestination<NodeId>,
    pub payload: Data,
    pub ttl: Option<TimeMetric>,
}

impl BinarySerde for Message {}
