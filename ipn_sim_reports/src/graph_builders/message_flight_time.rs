use crate::graph_builder::{GraphBuilder, render_mean_sd_graph};
use yew::Html;
use crate::reports::message_flight_time::MessageFlightTime as MessageFlightTimeReport;
use crate::format_time::format_time;
use ipn_sim_lib::utils::TimeMetric;

#[derive(Clone)]
pub struct MessageFlightTime;

impl GraphBuilder for MessageFlightTime {
    type Report = MessageFlightTimeReport;

    fn render_body(report: &Self::Report, scale_x: &dyn Fn(f32) -> f32, scale_y: &dyn Fn(f32) -> f32, domain_width: f32, domain_height: f32) -> Html {
        render_mean_sd_graph(
            &report.average_message_flight_times,
            &report.message_flight_time_std_devs,
            scale_x,
            scale_y,
            domain_width,
            domain_height,
        )
    }

    fn format_tick(tick_value: f32) -> String {
        format_time(tick_value as TimeMetric, Some(1))
    }

    fn y_max_value(report: &Self::Report) -> f32 {
        report.average_message_flight_times.history
            .iter()
            .zip(report.message_flight_time_std_devs.history.iter())
            .map(|((_, average), (_, std_dev))| *average + *std_dev)
            .fold(0., f32::max)
    }
}