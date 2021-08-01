use crate::utils::{MessageId, NodeId};

pub struct Binary {
    messages_to_spray: Vec<MessageToSpray>
}

struct MessageToSpray {
    id: MessageId,
    copies_remaining: u16,
    sprayed_node_ids: Vec<NodeId>
}

