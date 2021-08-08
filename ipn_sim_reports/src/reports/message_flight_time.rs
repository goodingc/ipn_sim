use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use yew::Html;

use ipn_sim_lib::event::Event;
use ipn_sim_lib::events::router_event::{RouterEvent, RouterEventType};
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::report::Report;
use ipn_sim_lib::utils::{MessageId, NodeId, Shared, TimeMetric};

use crate::graph_report::GraphReport;
use crate::utils::{destination_to_ids, mean_std_dev};
use crate::utils::format_time::format_time;
use crate::utils::paths::render_mean_sd_graph;
use crate::value_logger::ValueLogger;
use crate::time_series_report::TimeSeriesReport;

#[derive(Clone)]
pub struct MessageFlightTime {
    messages_in_flight: HashMap<MessageId, MessageInFlight>,
    message_flight_times: Vec<TimeMetric>,
    pub average_message_flight_times: ValueLogger<f32>,
    pub message_flight_time_std_devs: ValueLogger<f32>,
}

#[derive(Clone)]
struct MessageInFlight {
    sent_time: TimeMetric,
    remaining_destination_ids: HashSet<NodeId>,
}

impl MessageFlightTime {
    pub fn new() -> Self {
        Self {
            messages_in_flight: HashMap::new(),
            message_flight_times: vec![],
            average_message_flight_times: ValueLogger::new(0., true),
            message_flight_time_std_devs: ValueLogger::new(0., true),
        }
    }
}

impl Report for MessageFlightTime {
    fn on_tick(&mut self, sim: &IpnSim, events: &Vec<Box<dyn Event>>) {
        for event in events {
            if let Some(router_event) = event.downcast_ref::<RouterEvent>() {
                match &router_event.event_type {
                    RouterEventType::MessageCreated { id, destination, .. } => {
                        self.messages_in_flight.insert(*id, MessageInFlight {
                            sent_time: sim.time,
                            remaining_destination_ids: destination_to_ids(destination, sim),
                        });
                    }
                    RouterEventType::MessageDelivered { id, .. } => {
                        let message_in_flight = self.messages_in_flight.get_mut(id).unwrap();

                        message_in_flight.remaining_destination_ids.remove(&router_event.node.borrow().id);
                        if message_in_flight.remaining_destination_ids.is_empty() {
                            self.message_flight_times.push(sim.time - message_in_flight.sent_time);

                            let (average_message_flight_time, message_flight_time_std_dev) =
                                mean_std_dev(
                                    &self.message_flight_times
                                        .iter()
                                        .map(|time| *time as f32)
                                        .collect(),
                                );

                            self.average_message_flight_times.log_value(sim.time, average_message_flight_time);
                            self.message_flight_time_std_devs.log_value(sim.time, message_flight_time_std_dev);

                            self.messages_in_flight.remove(id);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl TimeSeriesReport for MessageFlightTime {
    fn render_body(
        &self,
        scale_x: &dyn Fn(f32) -> f32,
        scale_y: &dyn Fn(f32) -> f32,
        domain_width: f32,
        domain_height: f32,
    ) -> Html {
        render_mean_sd_graph(
            &self.average_message_flight_times,
            &self.message_flight_time_std_devs,
            scale_x,
            scale_y,
            domain_width,
            domain_height,
        )
    }

    fn format_tick(&self, tick_value: f32) -> String {
        format_time(tick_value as TimeMetric, Some(1))
    }

    fn y_max_value(&self) -> f32 {
        self.average_message_flight_times.history
            .iter()
            .zip(self.message_flight_time_std_devs.history.iter())
            .map(|((_, average), (_, std_dev))| *average + *std_dev)
            .fold(0., f32::max)
    }
}

impl GraphReport for MessageFlightTime {
    fn render_graph(&self, width: u16, height: u16, sim_time: TimeMetric) -> Html {
        TimeSeriesReport::render_graph(self, width, height, sim_time)
    }
}