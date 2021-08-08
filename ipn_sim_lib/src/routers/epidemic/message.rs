use crate::utils::{Data, MessageId, NodeId, TimeMetric};

use crate::binary_serde::BinarySerde;
use crate::events::create_message_event::MessageDestination;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct Message {
    pub id: MessageId,
    pub source_id: NodeId,
    pub destination: MessageDestination,
    pub payload: Data,
    pub ttl: Option<TimeMetric>,
}

impl BinarySerde for Message {}
