use crate::graph_builder::GraphBuilder;
use ipn_sim_lib::utils::{Shared, TimeMetric, shared};
use yew::prelude::*;
use crate::format_time::format_time;

const LEFT_MARGIN: u16 = 30;
const RIGHT_MARGIN: u16 = 5;
const TOP_MARGIN: u16 = 10;
const BOTTOM_MARGIN: u16 = 20;

const HORIZONTAL_TICK_SPACING: u16 = 30;
const VERTICAL_TICK_SPACING: u16 = 20;

pub struct Graph<B: GraphBuilder>(pub Shared<B::Report>);

impl<B: GraphBuilder> Graph<B> {
    pub fn new(report: B::Report) -> Self {
        Self(shared(report))
    }

    pub fn render_graph(&self, width: u16, height: u16, sim_time: TimeMetric) -> Html {
        let time = sim_time as f32;
        let y_max_value = B::y_max_value(&self.0.borrow());

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
                let text = B::format_tick(tick_value);
                html! {
                    <g transform=format!("translate(0, {})", transform_y)>
                        <line stroke="currentColor" x2="-6"></line>
                        <text fill="currentColor" x="-9" dy="0.32em">{text}</text>
                    </g>
                }
            })
            .collect::<Html>();

        let body = B::render_body(
            &self.0.borrow(),
            &scale_x,
            &scale_y,
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