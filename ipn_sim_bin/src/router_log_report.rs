use crate::format_time::format_time;
use ipn_sim_lib::event::Event;
use ipn_sim_lib::events::router_event::{RouterEvent, RouterEventType};
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::report::Report;
use std::fs::File;
use std::io::Write;
use ipn_sim_lib::message_destination::MessageDestination;
use ipn_sim_lib::utils::Shared;
use ipn_sim_lib::node::node::Node;

pub struct RouterLogReport(pub File);

impl Report for RouterLogReport {
    fn on_tick(&mut self, sim: &IpnSim, events: &Vec<Box<dyn Event>>) {
        events
            .iter()
            .filter_map(|event| event.downcast_ref::<RouterEvent>())
            .for_each(|event| {
                let details = match &event.event_type {
                    RouterEventType::Log(message) => message.clone(),
                    RouterEventType::MessageCreated {
                        id,
                        destination,
                        ttl,
                    } => format!(
                        "created message {} destined for {} with {}",
                        id,
                        message_destination_string(destination),
                        if let Some(ttl) = ttl {
                            format!("a ttl of {}", format_time(*ttl))
                        } else {
                            "no ttl".to_string()
                        }
                    ),
                    RouterEventType::MessageSent {
                        id,
                        destination_node,
                    } => format!("sent message {} to {}", id, destination_node.borrow().name),
                    RouterEventType::MessageReceived { id, source_node } => {
                        format!("received message {} from {}", id, source_node.borrow().name)
                    }
                    _ => String::new(),
                    // RouterEventType::MessageDropped { .. } => {}
                    // RouterEventType::MessageDelivered { .. } => {}
                };

                self.0
                    .write_all(
                        format!(
                            "[{}] [{}] {}\n",
                            format_time(sim.time),
                            event.node.borrow().name,
                            details
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            })
    }
}

fn message_destination_string(destination: &MessageDestination<Shared<Node>>) -> String {
    match destination {
        MessageDestination::<Shared<Node>>::All => String::from("all nodes"),
        MessageDestination::<Shared<Node>>::Single(node) => node.borrow().name.clone(),
        MessageDestination::<Shared<Node>>::Multiple(nodes) => nodes
            .iter()
            .map(|node| node.borrow().name.clone())
            .collect::<Vec<_>>()
            .join(", "),
    }
}
