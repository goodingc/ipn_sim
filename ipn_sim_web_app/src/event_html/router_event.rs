use std::iter;

use yew::prelude::*;

use ipn_sim_lib::events::router_event::{MessageDestination, RouterEvent, RouterEventType};

use crate::components::data::Data;
use crate::event_html::event_html::EventHtml;
use crate::utils::format_time::format_time;

impl EventHtml for RouterEvent {
    fn get_title(&self) -> &'static str {
        "Router"
    }

    fn get_details(&self) -> Vec<Html> {
        vec![
            Html::from(&self.node.borrow().name),
            match &self.event_type {
                RouterEventType::Log(message) => Html::from(message),
                RouterEventType::MessageCreated {
                    id,
                    destination,
                    ttl,
                } => html! {
                    <>
                    {"created a message with id "}
                    {id}
                    {" destined for "}
                    {message_destination_string(destination)}
                    {" with "}
                    {
                        if let Some(ttl) = ttl {
                            format!("a ttl of {}", format_time(*ttl, None))
                        } else {
                            "no ttl".to_string()
                        }
                    }
                    </>
                },
                RouterEventType::MessageSent {
                    id,
                    destination_node,
                } => html! {
                    <>
                    {"sent a message with id "}
                    {id}
                    {" to "}
                    {&destination_node.borrow().name}
                    </>
                },
                RouterEventType::MessageReceived { id, source_node } => html! {
                    <>
                    {"received a message with id "}
                    {id}
                    {" from "}
                    {&source_node.borrow().name}
                    </>
                },
                RouterEventType::MessageDelivered { id, source_node } => html! {
                    <>
                    {"delivered a message with id "}
                    {id}
                    {" from "}
                    {&source_node.borrow().name}
                    </>
                },
                _ => Html::from(""),
                // RouterEventType::MessageDropped { .. } => {}
            },
        ]
    }
}

fn message_destination_string(destination: &MessageDestination) -> String {
    match destination {
        MessageDestination::All => String::from("all nodes"),
        MessageDestination::Single(node) => node.borrow().name.clone(),
        MessageDestination::Multiple(nodes) => nodes
            .iter()
            .map(|node| node.borrow().name.clone())
            .collect::<Vec<_>>()
            .join(", "),
    }
}
