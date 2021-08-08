use crate::value_logger::ValueLogger;
use crate::graph_report::{GraphReport, render_mean_sd_graph};
use yew::Html;
use ipn_sim_lib::report::Report;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::event::Event;
use ipn_sim_lib::events::awake_router_event::AwakeRouterEvent;
use ipn_sim_lib::events::create_message_event::CreateMessageEvent;
use ipn_sim_lib::events::receive_data_event::ReceiveDataEvent;
use crate::utils::mean_std_dev;
use crate::graph_builder::GraphBuilder;
use crate::reports::message_buffer_occupancy::MessageBufferOccupancy as MessageBufferOccupancyReport;

#[derive(Clone)]
pub struct MessageBufferOccupancy;

impl GraphBuilder for MessageBufferOccupancy {
    type Report = MessageBufferOccupancyReport;

    fn render_body(report: &Self::Report, scale_x: &dyn Fn(f32) -> f32, scale_y: &dyn Fn(f32) -> f32, domain_width: f32, domain_height: f32) -> Html {
        render_mean_sd_graph(
            &report.average_message_buffer_occupancies,
            &report.message_buffer_occupancy_std_devs,
            scale_x,
            scale_y,
            domain_width,
            domain_height,
        )
    }

    fn y_max_value(report: &Self::Report) -> f32 {
        1.
    }
}