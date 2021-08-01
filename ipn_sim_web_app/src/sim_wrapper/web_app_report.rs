use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use ipn_sim_lib::event::Event;
use ipn_sim_lib::events::router_event::{MessageDestination, RouterEvent, RouterEventType};
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::report::Report;
use ipn_sim_lib::utils::{MessageId, NodeId, TimeMetric, shared, Shared};

use crate::sim_wrapper::interval_event::IntervalEvent;
use crate::sim_wrapper::log::Log;
use ipn_sim_lib::events::create_message_event::CreateMessageEvent;
use ipn_sim_lib::node::Node;
use std::iter;

pub struct WebAppReport {
    pub event_log: Log<TimeMetric, Box<dyn Event>>,
    pub router_log: Log<TimeMetric, RouterEvent>,
    pub sending_node_indices: HashMap<(usize, usize), bool>,
    pub creating_node_indices: Vec<usize>,
    pub delivering_node_indices: Vec<usize>,
}

pub struct MessageInFlight {
    sent_time: TimeMetric,
    alive: bool,
    remaining_destination_ids: HashSet<NodeId>,
    ttl: Option<TimeMetric>,
}

impl WebAppReport {
    pub fn new() -> Self {
        Self {
            event_log: Log::new(),
            router_log: Log::new(),
            sending_node_indices: HashMap::new(),
            creating_node_indices: vec![],
            delivering_node_indices: vec![],
        }
    }
}

impl Report for WebAppReport {
    fn on_tick(&mut self, sim: &IpnSim, events: &Vec<Box<dyn Event>>) {
        let reportable_events = events
            .iter()
            .filter(|event| !event.is::<IntervalEvent>())
            .map(|event| (*event).clone())
            .collect::<Vec<_>>();

        if !reportable_events.is_empty() {
            let time = sim.time;

            let router_events = reportable_events
                .iter()
                .filter_map(|event| {
                    event.downcast_ref::<RouterEvent>().map(|event| {
                        match &event.event_type {
                            RouterEventType::MessageCreated {
                                id,
                                destination,
                                ttl,
                            } => {
                                self.creating_node_indices
                                    .push(event.node.borrow().id as usize);
                            }
                            RouterEventType::MessageSent {
                                destination_node, ..
                            } => {
                                let source_node_index = event.node.borrow().id as usize;
                                let destination_node_index = destination_node.borrow().id as usize;

                                let node_indices = if source_node_index > destination_node_index {
                                    (source_node_index, destination_node_index)
                                } else {
                                    (destination_node_index, source_node_index)
                                };
                                self.sending_node_indices.insert(node_indices, true);
                                // data.sent_messages_count += 1;
                            }
                            RouterEventType::MessageReceived { source_node, .. } => {
                                let source_node_index = source_node.borrow().id as usize;
                                let destination_node_index = event.node.borrow().id as usize;

                                let node_indices = if source_node_index > destination_node_index {
                                    (source_node_index, destination_node_index)
                                } else {
                                    (destination_node_index, source_node_index)
                                };

                                self.sending_node_indices.insert(node_indices, false);
                            }
                            RouterEventType::MessageDelivered { id, .. } => {
                                let node_id = event.node.borrow().id;
                                self.delivering_node_indices.push(node_id as usize);
                            }
                            _ => {}
                        }
                        event.clone()
                    })
                })
                .collect::<Vec<_>>();

            self.event_log.add_events(time, reportable_events);

            if !router_events.is_empty() {
                self.router_log.add_events(time, router_events)
            }
        }
    }
}

fn mean_std_dev(values: &Vec<f32>) -> (f32, f32) {
    let mean = values.iter().sum::<f32>() / values.len() as f32;
    let std_dev = values
        .iter()
        .map(|value| (*value - mean).abs())
        .sum::<f32>()
        / values.len() as f32;

    (mean, std_dev)
}
