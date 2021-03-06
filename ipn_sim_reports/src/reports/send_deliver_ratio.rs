use std::rc::Rc;

use yew::prelude::*;

use ipn_sim_lib::events::router_event::{RouterEvent, RouterEventType};
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::report::Report;
use ipn_sim_lib::utils::{log, Shared, TimeMetric};

use crate::graph_report::GraphReport;
use crate::reports::message_states::MessageStates;
use crate::utils::paths::time_series_path;
use crate::value_logger::ValueLogger;
use crate::time_series_report::TimeSeriesReport;

#[derive(Clone)]
pub struct SendDeliverRatio {
    pub send_deliver_ratios: ValueLogger<f32>,
    sent_messages_count: u16,
    delivered_messages_count: u16,
}

impl SendDeliverRatio {
    pub fn new() -> Self {
        Self {
            send_deliver_ratios: ValueLogger::new(0., true),
            sent_messages_count: 0,
            delivered_messages_count: 0,
        }
    }
}

impl Report for SendDeliverRatio {
    fn on_tick(&mut self, sim: &IpnSim, events: &Vec<Box<dyn ipn_sim_lib::event::Event>>) {
        let mut ratio_changed = false;
        for event in events {
            if let Some(router_event) = event.downcast_ref::<RouterEvent>() {
                match router_event.event_type {
                    RouterEventType::MessageSent { .. } => {
                        self.sent_messages_count += 1;
                        ratio_changed = true;
                    }
                    RouterEventType::MessageDelivered { .. } => {
                        self.delivered_messages_count += 1;
                        ratio_changed = true;
                    }
                    _ => {}
                }
            }
        }
        if ratio_changed {
            self.send_deliver_ratios.log_value(
                sim.time,
                if self.delivered_messages_count == 0 {
                    0.
                } else {
                    self.sent_messages_count as f32 / self.delivered_messages_count as f32
                },
            );
        }
    }
}

impl TimeSeriesReport for SendDeliverRatio {
    fn render_body(
        &self,
        scale_x: &dyn Fn(f32) -> f32,
        scale_y: &dyn Fn(f32) -> f32,
        domain_width: f32,
        domain_height: f32,
    ) -> Html {
        let mut path = time_series_path(
            self.send_deliver_ratios.history.iter(),
            scale_x,
            scale_y,
        );
        path = format!("M 0 {} {} H {}", domain_height, path, domain_width);

        html! {
            <path
                fill="none"
                stroke="black"
                d=path
            ></path>
        }
    }

    fn y_max_value(&self) -> f32 {
        self.send_deliver_ratios.float_max_value()
    }
}

impl GraphReport for SendDeliverRatio {
    fn render_graph(&self, width: u16, height: u16, sim_time: TimeMetric) -> Html {
        TimeSeriesReport::render_graph(self, width, height, sim_time)
    }
}