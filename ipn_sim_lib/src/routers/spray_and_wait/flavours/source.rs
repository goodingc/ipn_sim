use crate::utils::{MessageId, NodeId};
use crate::routers::spray_and_wait::flavour::Flavour;
use crate::binary_serde::BinarySerde;
use crate::routers::spray_and_wait::message::Message;

#[derive(Clone)]
pub struct Source {
    messages_to_spray: Vec<MessageToSpray>,
    messages_to_forward: Vec<(MessageId, NodeId)>
}

struct MessageToSpray {
    id: MessageId,
    copies_remaining: u16,
    sprayed_node_ids: Vec<NodeId>
}

#[derive(Serialize, Deserialize, Hash, Clone)]
pub struct PingPacket {
    source_id: NodeId,
}

#[derive(Serialize, Deserialize, Hash, Clone)]
pub struct FulfillmentPacket {
    source_id: NodeId,
    destination_id: NodeId,
    messages: Vec<Message>,
}

impl Flavour for Source {
    type PingPacket = PingPacket;
    type FulfillmentPacket = ();

    fn new() -> Self {
        todo!()
    }
}

impl BinarySerde for PingPacket {}
impl BinarySerde for FulfillmentPacket {}