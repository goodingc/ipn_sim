use yew::prelude::*;

use ipn_sim_lib::utils::TimeMetric;

use crate::format_time::format_time;
use crate::value_logger::ValueLogger;

const LEFT_MARGIN: u16 = 30;
const RIGHT_MARGIN: u16 = 5;
const TOP_MARGIN: u16 = 10;
const BOTTOM_MARGIN: u16 = 20;

const HORIZONTAL_TICK_SPACING: u16 = 30;
const VERTICAL_TICK_SPACING: u16 = 20;

pub trait GraphReport {
    fn render_body(
        &self,
        scale_x: impl Fn(f32) -> f32 + Copy,
        scale_y: impl Fn(f32) -> f32 + Copy,
        domain_width: f32,
        domain_height: f32,
    ) -> Html;

    fn format_tick(tick_value: f32) -> String {
        tick_value.to_string()
    }

    fn y_max_value(&self) -> f32;

    fn render_graph(&self, width: u16, height: u16, sim_time: TimeMetric) -> Html {
        let time = sim_time as f32;
        let y_max_value = self.y_max_value();

        let x_tick_count = width / HORIZONTAL_TICK_SPACING;
        let y_tick_count = height / VERTICAL_TICK_SPACING;

        let domain_width = width.checked_sub(LEFT_MARGIN + RIGHT_MARGIN).unwrap_or(0) as f32;
        let domain_height = (height - TOP_MARGIN - BOTTOM_MARGIN) as f32;

        let scale_x = |value: f32| {
            let normalized_value = value / time;
            (if normalized_value.is_nan() {
                0.
            } else {
                normalized_value
            }) * domain_width
        };

        let scale_y = |value: f32| {
            let normalized_value = value / y_max_value;
            (1. - if normalized_value.is_nan() {
                0.
            } else {
                normalized_value
            }) * domain_height
        };

        let x_ticks_html = axis_ticks::ticks(
            0.,
            time,
            x_tick_count as usize,
        ).iter()
            .map(|&tick_time| {
                let transform_x = scale_x(tick_time) + 0.5;
                html! {
                        <g transform=format!("translate({}, 0)", transform_x)>
                            <line stroke="currentColor" y2="6"></line>
                            <text fill="currentColor" y="9" dy="0.71em">{format_time(tick_time as TimeMetric, Some(1))}</text>
                        </g>
                    }
            }).collect::<Html>();

        let y_ticks_html = axis_ticks::ticks(0., y_max_value, y_tick_count as usize)
            .iter()
            .map(|&tick_value| {
                let transform_y = scale_y(tick_value) + 0.5;
                let text = Self::format_tick(tick_value);
                html! {
                        <g transform=format!("translate(0, {})", transform_y)>
                            <line stroke="currentColor" x2="-6"></line>
                            <text fill="currentColor" x="-9" dy="0.32em">{text}</text>
                        </g>
                    }
            })
            .collect::<Html>();

        let body = self.render_body(
            scale_x,
            scale_y,
            domain_width,
            domain_height,
        );

        html! {
            <svg style="width: 100%; height: 200px">
                <g transform=format!("translate({}, {})", LEFT_MARGIN, height - BOTTOM_MARGIN) fill="none" font-size="10" text-anchor="middle">
                    <path
                        stroke="currentColor"
                        d=format!("M 0.5 6 V 0.5 H {}.5 V 6", domain_width)
                    ></path>
                    {x_ticks_html}
                </g>
                <g transform=format!("translate({}, {})", LEFT_MARGIN, TOP_MARGIN) fill="none" font-size="10" text-anchor="end">
                    <path
                        stroke="currentColor"
                        d=format!("M -6 0.5 H 0.5 V {}.5 H -6", domain_height)
                    ></path>
                    {y_ticks_html}
                    {body}
                </g>
            </svg>
        }
    }
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