use std::collections::{HashMap, HashSet};
use std::iter;
use std::rc::Rc;

use upcast::Upcast;
use yew::prelude::*;

use ipn_sim_lib::event::Event;
use ipn_sim_lib::events::router_event::{MessageDestination, RouterEvent, RouterEventType};
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::report::Report;
use ipn_sim_lib::utils::{MessageId, NodeId, Shared, TimeMetric};

use crate::graph_report::GraphReport;
use crate::utils::paths::value_path;
use crate::value_logger::ValueLogger;
use crate::time_series_report::TimeSeriesReport;

#[derive(Clone)]
pub struct MessageStates {
    messages_in_flight: HashMap<MessageId, MessageInFlight>,
    pub created_message_counts: ValueLogger<u16>,
    pub delivered_message_counts: ValueLogger<u16>,
    pub dropped_message_counts: ValueLogger<u16>,
}

#[derive(Clone)]
struct MessageInFlight {
    sent_time: TimeMetric,
    alive: bool,
    remaining_destination_ids: HashSet<NodeId>,
    ttl: Option<TimeMetric>,
}

impl MessageStates {
    pub fn new() -> Self {
        Self {
            messages_in_flight: HashMap::new(),
            created_message_counts: ValueLogger::new(0, true),
            delivered_message_counts: ValueLogger::new(0, true),
            dropped_message_counts: ValueLogger::new(0, true),
        }
    }
}

impl Report for MessageStates {
    fn on_tick(&mut self, sim: &IpnSim, events: &Vec<Box<dyn Event>>) {
        let mut just_dropped_messages = 0;
        for message_in_flight in self.messages_in_flight.values_mut() {
            if !message_in_flight.alive {
                continue;
            }
            if let Some(ttl) = message_in_flight.ttl {
                if ttl <= sim.time {
                    message_in_flight.alive = false;
                    just_dropped_messages += 1;
                }
            }
        }
        if just_dropped_messages > 0 {
            // self.dropped_message_counts.log_value(sim.time, self.dropped_message_counts.value);
            self.dropped_message_counts.log_value(sim.time, self.dropped_message_counts.value + just_dropped_messages);
        }

        for event in events {
            if let Some(router_event) = event.downcast_ref::<RouterEvent>() {
                match &router_event.event_type {
                    RouterEventType::MessageCreated {
                        id,
                        destination,
                        ttl
                    } => {
                        // self.created_message_counts.log_value(sim.time, self.created_message_counts.value);
                        self.created_message_counts.log_value(sim.time, self.created_message_counts.value + 1);
                        self.messages_in_flight.insert(
                            *id,
                            MessageInFlight {
                                sent_time: sim.time,
                                alive: true,
                                remaining_destination_ids: match destination {
                                    MessageDestination::All =>
                                        sim.nodes.as_ref().unwrap().iter().map(|node| node.borrow().id).collect(),
                                    MessageDestination::Single(node) =>
                                        iter::once(node.borrow().id).collect(),
                                    MessageDestination::Multiple(nodes) =>
                                        nodes.iter().map(|node| node.borrow().id).collect()
                                },
                                ttl: *ttl,
                            },
                        );
                    }
                    RouterEventType::MessageDelivered { id, .. } => {
                        let message_in_flight = self.messages_in_flight.get_mut(id).unwrap();

                        message_in_flight.remaining_destination_ids.remove(&router_event.node.borrow().id);
                        if message_in_flight.remaining_destination_ids.is_empty() {
                            if message_in_flight.alive {
                                // self.delivered_message_counts.log_value(sim.time, self.delivered_message_counts.value);
                                self.delivered_message_counts.log_value(sim.time, self.delivered_message_counts.value + 1);
                            }
                            self.messages_in_flight.remove(id);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl TimeSeriesReport for MessageStates {
    fn render_body(
        &self,
        scale_x: &dyn Fn(f32) -> f32,
        scale_y: &dyn Fn(f32) -> f32,
        domain_width: f32,
        domain_height: f32,
    ) -> Html {
        let created_area = format!(
            "M 0 {} {} H {} V {}",
            domain_height,
            value_path(
                &self.created_message_counts,
                scale_x,
                scale_y,
            ),
            domain_width,
            domain_height
        );

        let dropped_area = format!(
            "M 0 {} {} H {} V {}",
            domain_height,
            value_path(
                &self.delivered_message_counts
                    .combine(
                        &self.dropped_message_counts,
                        |&delivered_count, &dropped_count| delivered_count + dropped_count,
                        ValueLogger::new(0, true),
                    ),
                scale_x,
                scale_y,
            ),
            domain_width,
            domain_height
        );


        let delivered_area = format!(
            "M 0 {} {} H {} V {}",
            domain_height,
            value_path(
                &self.delivered_message_counts,
                scale_x,
                scale_y,
            ),
            domain_width,
            domain_height
        );

        html! {
            <>
            <path
                fill="var(--bs-warning)"
                d=created_area
            ></path>
            <path
                fill="var(--bs-danger)"
                d=dropped_area
            ></path>
            <path
                fill="var(--bs-success)"
                d=delivered_area
            ></path>
            </>
        }
    }

    fn format_tick(&self, tick_value: f32) -> String {
        (tick_value as u32).to_string()
    }

    fn y_max_value(&self) -> f32 {
        self.created_message_counts
            .max_value() as f32
    }
}

impl GraphReport for MessageStates {
    fn render_graph(&self, width: u16, height: u16, sim_time: TimeMetric) -> Html {
        TimeSeriesReport::render_graph(self, width, height, sim_time)
    }
}