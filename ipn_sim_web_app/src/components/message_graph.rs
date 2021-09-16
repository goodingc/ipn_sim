use yew::prelude::*;
use ipn_sim_reports::reports::messages::Message;
use ipn_sim_lib::utils::{MessageId, Shared};
use crate::sim_wrapper::sim_wrapper::SimWrapper;
use ipn_sim_lib::cgmath::{Vector2, Point2};
use num_traits::Zero;
use web_sys::Element;
use std::rc::Rc;
use yew::services::render::RenderTask;
use yew::services::RenderService;
use num_traits::real::Real;
use ipn_sim_lib::message_destination::IsIncluded;

pub struct MessageGraph {
    link: ComponentLink<Self>,
    props: Props,
    svg_ref: NodeRef,
    size: Vector2<f32>,
    origin: Vector2<f32>,
    render_task: RenderTask,
    separation_velocity: f32
}

// impl MessageGraph {
//     fn vertex_screen_position(&self) -> Vector2<f32> {
//
//     }
// }

#[derive(Properties, Clone)]
pub struct Props {
    pub message_id: MessageId,
    pub wrapper: Shared<SimWrapper>,
}

pub enum ComponentMessage {
    Tick
}

impl Component for MessageGraph {
    type Message = ComponentMessage;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let render_task = RenderService::request_animation_frame(
            link.callback(|_| Self::Message::Tick)
        );
        Self {
            props,
            link,
            svg_ref: NodeRef::default(),
            size: Vector2::zero(),
            origin: Vector2::zero(),
            render_task,
            separation_velocity: 0.
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ComponentMessage::Tick => {
                let wrapper = self.props.wrapper
                    .borrow();

                let mut report = wrapper.single_message_graph_report
                    .borrow_mut();

                if let Some(data) = &mut report.active_data {
                    let overshoot = data.graph_layout.vertices
                        .iter()
                        .fold(f32::min_value(), |overshoot, vertex| {
                            let Point2 {
                                x, y
                            } = vertex.borrow().position;

                            overshoot.max(x.abs() - self.origin.x)
                                .max(y.abs() - self.origin.y)
                        }) + 10.;

                    self.separation_velocity -= overshoot * 0.001;
                    self.separation_velocity *= 0.9;

                    data.graph_layout.target_separation += self.separation_velocity;

                    data.graph_layout.step();
                }

                self.render_task = RenderService::request_animation_frame(
                    self.link.callback(|_| Self::Message::Tick)
                );
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.message_id != self.props.message_id {
            props.wrapper
                .borrow().single_message_graph_report
                .borrow_mut().set_active_data(props.message_id);
            self.separation_velocity = 0.;
            self.props = props;
            true
        } else {
            false
        }

    }

    fn view(&self) -> Html {
        let wrapper = self.props.wrapper.borrow();
        let report = wrapper.single_message_graph_report.borrow();

        report.active_data
            .as_ref()
            .map_or("".into(), |data| {
                let message_data = report.message_data
                    .get(&self.props.message_id)
                    .unwrap();

                let vertices_html = data.node_vertices
                    .iter()
                    .map(|(node, vertex)| {
                        let screen_position = vertex.borrow().position + self.origin;

                        let mut color = if Rc::ptr_eq(&message_data.source_node, node) {
                            "green"
                        } else if message_data.destination.is_included(node) {
                            "red"
                        } else {
                            "blue"
                        };

                        html! {
                            <>
                            <circle fill=color cx=screen_position.x.to_string() cy=screen_position.y.to_string() r="10"/>
                            <text
                                x=screen_position.x.to_string()
                                y=screen_position.y.to_string()
                                dominant-baseline="middle"
                                text-anchor="middle"
                                font-size="small"
                            >{ &node.borrow().name }</text>
                            </>

                        }
                    }).collect::<Html>();

                let edges_html = data.graph_layout.edges
                    .iter()
                    .map(|(vertex_1, vertex_2)| {
                        let v_1_pos = vertex_1.borrow().position + self.origin;
                        let v_2_pos = vertex_2.borrow().position + self.origin;
                        html! {
                            <path marker-end="url(#head)" fill="none" stroke="black" d=format!("M{},{}L{},{}", v_1_pos.x, v_1_pos.y, v_2_pos.x, v_2_pos.y)/>
                        }
                    }).collect::<Html>();

                html! {
                    <svg
                        ref=self.svg_ref.clone()
                        style="width: 100%; height: calc(25vw - 2em)"
                    >
                        <defs>
                            <marker id="head" orient="auto" markerWidth="10" markerHeight="10" refX="20" refY="5">
                                <line stroke="black" x1="10" y1="5" x2="0" y2="0"/>
                                <line stroke="black" x1="10" y1="5" x2="0" y2="10"/>
                            </marker>
                        </defs>
                        {edges_html}
                        {vertices_html}
                    </svg>
                }
            })
    }

    fn rendered(&mut self, first_render: bool) {
        let size = self.svg_ref
            .cast::<Element>()
            .map_or(Vector2::new(0., 0.), |svg_element|
                Vector2::new(
                    svg_element.client_width() as f32,
                    svg_element.client_height() as f32,
                ),
            );

        if size != self.size {
            self.size = size;
            self.origin = size / 2.;
        }
    }
}