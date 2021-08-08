use std::rc::Rc;

use yew::Html;

use ipn_sim_lib::event::Event;
use ipn_sim_lib::events::awake_router_event::AwakeRouterEvent;
use ipn_sim_lib::events::create_message_event::CreateMessageEvent;
use ipn_sim_lib::events::receive_data_event::ReceiveDataEvent;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::report::Report;
use ipn_sim_lib::utils::{Shared, TimeMetric};

use crate::graph_report::GraphReport;
use crate::utils::mean_std_dev;
use crate::utils::paths::render_mean_sd_graph;
use crate::value_logger::ValueLogger;
use crate::time_series_report::TimeSeriesReport;

#[derive(Clone)]
pub struct MessageBufferOccupancy {
    pub average_message_buffer_occupancies: ValueLogger<f32>,
    pub message_buffer_occupancy_std_devs: ValueLogger<f32>,
    pub occupancies: Vec<f32>,
}

impl MessageBufferOccupancy {
    pub fn new() -> Self {
        Self {
            average_message_buffer_occupancies: ValueLogger::new(0., true),
            message_buffer_occupancy_std_devs: ValueLogger::new(0., true),
            occupancies: vec![],
        }
    }
}

impl Report for MessageBufferOccupancy {
    fn on_tick(&mut self, sim: &IpnSim, events: &Vec<Box<dyn Event>>) {
        let mut meaningful_tick = false;
        for event in events {
            if event.is::<AwakeRouterEvent>() || event.is::<CreateMessageEvent>() || event.is::<ReceiveDataEvent>() {
                meaningful_tick = true;
                break;
            }
        }
        if !meaningful_tick {
            return;
        }
        self.occupancies = sim
            .nodes
            .as_ref()
            .unwrap()
            .iter()
            .map(|node| node.borrow().message_buffer.get_occupancy())
            .collect::<Vec<_>>();

        let (average_message_buffer_occupancy, message_buffer_occupancy_std_dev) =
            mean_std_dev(&self.occupancies);

        self.average_message_buffer_occupancies.log_value(sim.time, average_message_buffer_occupancy);
        self.message_buffer_occupancy_std_devs.log_value(sim.time, message_buffer_occupancy_std_dev);
    }
}

impl TimeSeriesReport for MessageBufferOccupancy {
    fn render_body(
        &self,
        scale_x: &dyn Fn(f32) -> f32,
        scale_y: &dyn Fn(f32) -> f32,
        domain_width: f32,
        domain_height: f32,
    ) -> Html {
        render_mean_sd_graph(
            &self.average_message_buffer_occupancies,
            &self.message_buffer_occupancy_std_devs,
            scale_x,
            scale_y,
            domain_width,
            domain_height,
        )
    }

    fn y_max_value(&self) -> f32 {
        1.
    }
}

impl GraphReport for MessageBufferOccupancy {
    fn render_graph(&self, width: u16, height: u16, sim_time: TimeMetric) -> Html {
        TimeSeriesReport::render_graph(self, width, height, sim_time)
    }
}