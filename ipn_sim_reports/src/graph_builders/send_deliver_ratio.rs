use crate::graph_builder::{GraphBuilder, time_series_path};
use yew::prelude::*;
use crate::reports::send_deliver_ratio::SendDeliverRatio as SendDeliverRatioReport;

#[derive(Clone)]
pub struct SendDeliverRatio;

impl GraphBuilder for SendDeliverRatio {
    type Report = SendDeliverRatioReport;

    fn render_body(report: &Self::Report, scale_x: &dyn Fn(f32) -> f32, scale_y: &dyn Fn(f32) -> f32, domain_width: f32, domain_height: f32) -> Html {
        let mut path = time_series_path(
            report.send_deliver_ratios.history.iter(),
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

    fn y_max_value(report: &Self::Report) -> f32 {
        report.send_deliver_ratios.float_max_value()
    }
}