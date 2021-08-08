use yew::prelude::*;

use ipn_sim_lib::utils::TimeMetric;

use crate::value_logger::ValueLogger;
use itertools::Itertools;

// pub fn time_series_path<'a, T: Into<f32> + Copy + 'a>(
//     iter: impl Iterator<Item=&'a (TimeMetric, T)>,
//     scale_x: impl Fn(f32) -> f32 + Copy,
//     scale_y: impl Fn(f32) -> f32 + Copy,
// ) -> String {
//     let mut prev_point: Option<(f32, f32)> = None;
//     iter
//         .step_by(2)
//         .map(|(time, value)| {
//         let (time, value) = (*time as f32, (*value).into());
//         let element = prev_point
//             .and_then(|(prev_time, prev_value)|
//                 if time == prev_time {
//                     Some(format!("V{:.1}", scale_y(value)))
//                 } else if value == prev_value {
//                     Some(format!("H{:.1}", scale_x(time)))
//                 } else {
//                     None
//                 }).unwrap_or_else(|| format!("L{:.1} {:.1}", scale_x(time), scale_y(value)));
//         prev_point = Some((time, value));
//         element
//     }).collect::<Vec<_>>()
//         .join("")
// }

pub fn time_series_path<'a, T: Into<f32> + Copy + 'a>(
    iter: impl Iterator<Item=&'a (TimeMetric, T)>,
    scale_x: impl Fn(f32) -> f32 + Copy,
    scale_y: impl Fn(f32) -> f32 + Copy,
) -> String {
    iter.fold(vec![], |mut points: Vec<(f32, f32)>, (time, value)| {
        let (x, y) = (scale_x(*time as f32), scale_y((*value).into()));
        let should_push = points.last().map_or(true, |(last_x, last_y)| {
            // ((x - *last_x).powi(2) + (y - *last_y).powi(2)).sqrt() > 5.
            //     // &&
                (x - *last_x) > 1.
        });
        if should_push {
            points.push((x, y));
        }
        points
    }).iter()
        .map(|(x, y)| format!("L{:.0} {:.0}", *x, *y)).join("")
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
