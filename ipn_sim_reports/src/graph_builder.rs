use ipn_sim_lib::report::Report;
use yew::prelude::*;
use crate::value_logger::ValueLogger;
use ipn_sim_lib::utils::TimeMetric;

pub trait GraphBuilder {
    type Report: Report;

    fn render_body(
        report: &Self::Report,
        scale_x: &dyn Fn(f32) -> f32,
        scale_y: &dyn Fn(f32) -> f32,
        domain_width: f32,
        domain_height: f32,
    ) -> Html;

    fn format_tick(tick_value: f32) -> String {
        tick_value.to_string()
    }

    fn y_max_value(report: &Self::Report) -> f32;
}

pub fn value_path<T: Copy + PartialEq + Into<f32>>(
    logger: &ValueLogger<T>,
    scale_x: impl Fn(f32) -> f32 + Copy,
    scale_y: impl Fn(f32) -> f32 + Copy,
) -> String {
    time_series_path(logger.history.iter(), scale_x, scale_y)
}

pub fn average_std_dev_paths(
    average_logger: &ValueLogger<f32>,
    std_dev_logger: &ValueLogger<f32>,
    scale_x: impl Fn(f32) -> f32 + Copy,
    scale_y: impl Fn(f32) -> f32 + Copy,
) -> (String, String, String) {
    let (plus_std_dev_data, minus_std_dev_data): (Vec<_>, Vec<_>) = average_logger
        .history
        .iter()
        .zip(std_dev_logger.history.iter())
        .map(|((time, average), (_, std_dev))| {
            ((*time, average + std_dev), (*time, average - std_dev))
        })
        .unzip();
    (
        time_series_path(plus_std_dev_data.iter(), scale_x, scale_y),
        time_series_path(average_logger.history.iter(), scale_x, scale_y),
        time_series_path(minus_std_dev_data.iter(), scale_x, scale_y),
    )
}

pub fn time_series_path<'a, T: Into<f32> + Copy + 'a>(
    iter: impl Iterator<Item=&'a (TimeMetric, T)>,
    scale_x: impl Fn(f32) -> f32 + Copy,
    scale_y: impl Fn(f32) -> f32 + Copy,
) -> String {
    iter.map(|(time, value)| format!("L {} {}", scale_x(*time as f32), scale_y((*value).into())))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn render_mean_sd_graph(
    average_logger: &ValueLogger<f32>,
    std_dev_logger: &ValueLogger<f32>,
    scale_x: impl Fn(f32) -> f32 + Copy,
    scale_y: impl Fn(f32) -> f32 + Copy,
    domain_width: f32,
    domain_height: f32,
) -> Html {
    let (mut plus_sd_path, mut average_path, mut minus_sd_path) = average_std_dev_paths(
        average_logger,
        std_dev_logger,
        scale_x,
        scale_y,
    );

    plus_sd_path = format!("M 0 {} {} H {}", domain_height, plus_sd_path, domain_width);
    average_path = format!("M 0 {} {} H {}", domain_height, average_path, domain_width);
    minus_sd_path = format!("M 0 {} {} H {}", domain_height, minus_sd_path, domain_width);

    html! {
        <>
        <path
            fill="none"
            stroke="grey"
            stroke-dasharray="5,10"
            d=plus_sd_path
        ></path>
        <path
            fill="none"
            stroke="black"
            d=average_path
        ></path>
        <path
            fill="none"
            stroke="grey"
            stroke-dasharray="5,10"
            d=minus_sd_path
        ></path>
        </>
    }
}