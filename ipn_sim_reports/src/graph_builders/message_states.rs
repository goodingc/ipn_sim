use crate::graph_builder::{GraphBuilder, value_path};
use yew::prelude::*;
use crate::reports::message_states::MessageStates as MessageStatesReport;
use crate::value_logger::ValueLogger;

#[derive(Clone)]
pub struct MessageStates;

impl GraphBuilder for MessageStates {
    type Report = MessageStatesReport;

    fn render_body(report: &Self::Report, scale_x: &dyn Fn(f32) -> f32, scale_y: &dyn Fn(f32) -> f32, domain_width: f32, domain_height: f32) -> Html {
        let created_area = format!(
            "M 0 {} {} H {} V {}",
            domain_height,
            value_path(
                &report.created_message_counts,
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
                &report.delivered_message_counts
                    .combine(
                        &report.dropped_message_counts,
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
                &report.delivered_message_counts,
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

    fn format_tick(tick_value: f32) -> String {
        (tick_value as u32).to_string()
    }

    fn y_max_value(report: &Self::Report) -> f32 {
        report.created_message_counts.max_value() as f32
    }
}