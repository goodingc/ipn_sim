use yew::prelude::*;
use ipn_sim_lib::utils::TimeMetric;

use crate::utils::format_time::format_time;

const LEFT_MARGIN: u16 = 30;
const RIGHT_MARGIN: u16 = 5;
const TOP_MARGIN: u16 = 10;
const BOTTOM_MARGIN: u16 = 20;

const HORIZONTAL_TICK_SPACING: u16 = 30;
const VERTICAL_TICK_SPACING: u16 = 20;

pub trait TimeSeriesReport {
    fn render_body(
        &self,
        scale_x: &dyn Fn(f32) -> f32,
        scale_y: &dyn Fn(f32) -> f32,
        domain_width: f32,
        domain_height: f32,
    ) -> Html;

    fn format_tick(&self, tick_value: f32) -> String {
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
                            <line stroke="black" y2="6"/>
                            <line stroke="lightgray" y2=(-domain_height).to_string()/>
                            <text fill="black" y="9" dy="0.71em">{format_time(tick_time as TimeMetric, Some(1))}</text>
                        </g>
                    }
            }).collect::<Html>();

        let y_ticks_html = axis_ticks::ticks(0., y_max_value, y_tick_count as usize)
            .iter()
            .map(|&tick_value| {
                let transform_y = scale_y(tick_value) + 0.5;
                let text = self.format_tick(tick_value);
                let gridline_html = if tick_value == 0. {
                    "".into()
                } else {
                    html! {
                        <line stroke="lightgray" x2=domain_width.to_string()/>
                    }
                };
                html! {
                        <g transform=format!("translate(0, {})", transform_y)>
                            <line stroke="black" x2="-6"></line>
                            {gridline_html}
                            <text fill="black" x="-9" dy="0.32em">{text}</text>
                        </g>
                    }
            })
            .collect::<Html>();

        let body = self.render_body(
            &scale_x,
            &scale_y,
            domain_width,
            domain_height,
        );

        html! {
            <svg style=format!("width: {}px; height: {}px", width, height)>
                <g transform=format!("translate({}, {})", LEFT_MARGIN, height - BOTTOM_MARGIN) fill="none" font-size="10" text-anchor="middle">
                    {x_ticks_html}
                    <path
                        stroke="black"
                        d=format!("M 0.5 6 V 0.5 H {}.5 V 6", domain_width)
                    />
                </g>
                <g transform=format!("translate({}, {})", LEFT_MARGIN, TOP_MARGIN) fill="none" font-size="10" text-anchor="end">
                    {y_ticks_html}
                    {body}
                    <path
                        stroke="black"
                        d=format!("M -6 0.5 H 0.5 V {}.5 H -6", domain_height)
                    />
                </g>
            </svg>
        }
    }
}