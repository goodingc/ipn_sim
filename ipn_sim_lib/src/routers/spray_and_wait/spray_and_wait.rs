// use crate::router::Router;
// use crate::utils::{NodeId, Data, TimeMetric, MessageId};
// use crate::routers::spray_and_wait::flavour::Flavour;
// use crate::router_link::RouterLink;
// use crate::events::create_message_event::MessageDestination;
// use bit_vec::BitBlock;
// use crate::routers::spray_and_wait::message::Message;
//
// pub struct SprayAndWait<F: Flavour> {
//     pub node_id: Option<NodeId>,
//     next_message_index: u16,
//     messages_to_spray: Vec<MessageToSpray>,
//     messages_to_forward: Vec<(MessageId, NodeId)>,
//     flavour: F
// }
//
// struct MessageToSpray {
//     id: MessageId,
//     copies_remaining: u16,
//     sprayed_node_ids: Vec<NodeId>
// }
//
// struct MessageToForward {
//     id: MessageId,
//
// }
//
// impl<F: Flavour> SprayAndWait<F> {
//     pub fn new() -> Self {
//         Self {
//             node_id: None,
//             next_message_index: 0,
//             messages_to_spray: vec![],
//             messages_to_forward: vec![],
//             flavour: F::new(),
//         }
//     }
// }
//
// impl<F: Flavour> Router for SprayAndWait<F> {
//     fn on_init(&mut self, _link: &mut RouterLink, id: NodeId) {
//         self.node_id = Some(id);
//     }
//
//     fn on_message_created(&mut self, link: &mut RouterLink, destination: MessageDestination, payload: Data, ttl: Option<TimeMetric>) {
//         let message_id = (self.node_id.unwrap() as MessageId)
//             .overflowing_shl((NodeId::bytes() as u32) * 8)
//             .0 | self.next_message_index as MessageId;
//         self.next_message_index += 1;
//         link.report_message_created(message_id, destination.clone(), ttl);
//         let message = Message {
//             id: message_id,
//             source_id: self.node_id.unwrap(),
//             destination,
//             payload,
//             ttl,
//         };
//
//     }
// }