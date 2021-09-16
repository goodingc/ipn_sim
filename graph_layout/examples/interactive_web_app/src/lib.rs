use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew::services::{*, interval::IntervalTask};
use web_sys::Element;
use std::time::Duration;
use yew::services::render::RenderTask;
use graph_layout::graph_layout::GraphLayout;
use graph_layout::cgmath::{Point2, Vector2, MetricSpace, Zero, InnerSpace, EuclideanSpace};
use graph_layout::Shared;
use graph_layout::vertex::Vertex;
use std::rc::Rc;
use crate::NewVertexData::AutoPositioned;
use std::collections::VecDeque;
use itertools::Itertools;
use petgraph::Graph;
use petgraph::graph::NodeIndex;

#[macro_use]
macro_rules! param_changer_callback {
    ($self:ident, $attr:ident) => {
        $self.link.callback(|data: InputData|
            Message::ChangeParameter(
                Rc::new(
                    |graph_layout| &mut graph_layout.$attr
                ),
                data,
            )
        )
    };
}

const VERTEX_RADIUS: f32 = 10.;
const GRAPH_FRAMES: usize = 200;

enum Message {
    Tick,
    MouseUp,
    MouseDown(Point2<i32>),
    MouseMove(Point2<i32>),
    ConfirmNewVertex,
    CancelNewVertex,
    StartAutoPosition,
    ChangeParameter(Rc<dyn Fn(&mut GraphLayout) -> &mut f32>, InputData),
}

struct App {
    link: ComponentLink<Self>,
    render_task: RenderTask,
    svg_ref: NodeRef,
    graph_layout: GraphLayout,
    size: Vector2<f32>,
    origin: Vector2<f32>,
    new_vertex: Option<NewVertexData>,
    dragging_vertex: Option<NodeIndex>,
    average_magnitudes: VecDeque<(f32, f32, f32)>,
    settled: bool,
}

enum NewVertexData {
    Positioned(Point2<f32>, Vec<NodeIndex>),
    AutoPositioned(Vec<NodeIndex>),
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let grid_size = 10;

        let mut edges = vec![(1,2)];

        // for i in 0..(grid_size - 1) {
        //     for j in 0..(grid_size - 1) {
        //         edges.push(((i, j), (i, j + 1)));
        //         edges.push(((i, j), (i + 1, j)));
        //     }
        //     edges.push(((i, grid_size - 1), (i + 1, grid_size - 1)));
        //     edges.push(((grid_size - 1, i), (grid_size - 1, i + 1)));
        // }

        let mut graph_layout = GraphLayout::new(
            Graph::from_edges(&edges),
            100.,
            0.1,
            0.3,
            1.8
        );

        Self {
            svg_ref: NodeRef::default(),
            render_task: RenderService::request_animation_frame(
                link.callback(|_| Self::Message::Tick)
            ),
            link,
            graph_layout,
            size: Vector2::new(0., 0.),
            origin: Vector2::new(0., 0.),
            new_vertex: None,
            dragging_vertex: None,
            average_magnitudes: VecDeque::new(),
            settled: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Tick => {
                if self.new_vertex.is_none() {
                    self.settled = self.graph_layout.step();

                    let (
                        total_displacement,
                        total_velocity,
                        total_acceleration
                    ) = self.graph_layout.graph
                        .node_weights()
                        .fold(Default::default(), |(s, v, a): (f32, f32, f32), vertex| {
                            (
                                s + vertex.position.to_vec().magnitude(),
                                v + vertex.velocity.magnitude(),
                                a + vertex.acceleration.magnitude()
                            )
                        });

                    let vertex_count = self.graph_layout.graph.node_count() as f32;

                    self.average_magnitudes.push_front((
                        total_displacement / vertex_count,
                        total_velocity / vertex_count,
                        total_acceleration / vertex_count
                    ));

                    if self.average_magnitudes.len() > GRAPH_FRAMES {
                        self.average_magnitudes.pop_back();
                    }
                }
                self.render_task = RenderService::request_animation_frame(
                    self.link.callback(|_| Self::Message::Tick)
                );
            }
            Message::MouseUp => {
                if let Some(index) = self.dragging_vertex {
                    self.graph_layout.graph[index].frozen = false;
                }
            }
            Message::MouseDown(position) => {
                let pressed_graph_position = position.cast().unwrap() - self.origin;

                let clicked = self.graph_layout.graph
                    .node_indices()
                    .find_map(|index|  {
                        let vertex = &mut self.graph_layout.graph[index];
                        (vertex.position.distance2(pressed_graph_position) <
                            VERTEX_RADIUS.powi(2)).then(|| {
                            (index, vertex)
                        })
                    });

                if let Some(new_vertex_data) = &mut self.new_vertex {
                    match new_vertex_data {
                        NewVertexData::Positioned(vertex_position, connections) => {
                            if let Some((clicked_index, _)) = clicked {
                                connections.push(clicked_index)
                            } else {
                                *vertex_position = pressed_graph_position;
                            }
                        }
                        NewVertexData::AutoPositioned(connections) => {
                            if let Some((clicked_index, _)) = clicked {
                                connections.push(clicked_index)
                            }
                        }
                    }
                } else {
                    if let Some((clicked_index, clicked_vertex)) = clicked {
                        clicked_vertex.frozen = true;
                        self.dragging_vertex = Some(clicked_index);
                    } else {
                        self.new_vertex = Some(NewVertexData::Positioned(pressed_graph_position, vec![]));
                    }
                }
            }
            Message::MouseMove(position) => {
                let graph_position = position.cast().unwrap() - self.origin;
                if let Some(dragging_vertex) = self.dragging_vertex {
                    self.graph_layout.graph[dragging_vertex].position = graph_position;
                }
            }
            Message::ConfirmNewVertex => {
                if let Some(data) = self.new_vertex.take() {
                    match data {
                        NewVertexData::Positioned(position, connections) => {
                            let index = self.graph_layout.graph.add_node(Vertex::new(position));

                            for connection in connections {
                                self.graph_layout.graph.add_edge(index, connection, ());
                            }
                        }
                        NewVertexData::AutoPositioned(connections) => {
                            // self.graph_layout.add_connected_vertex(connections);
                        }
                    }
                }
            }
            Message::CancelNewVertex => {
                self.new_vertex = None;
            }
            Message::StartAutoPosition => {
                self.new_vertex = Some(NewVertexData::AutoPositioned(vec![]));
            }
            Message::ChangeParameter(extractor, data) => {
                *extractor(&mut self.graph_layout) = data.value.parse().unwrap();
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let vertices_html = self.graph_layout.graph
            .node_weights()
            .map(|vertex| {
                let screen_position = vertex.position + self.origin;

                let mut color = "black";

                if let Some(data) = &self.new_vertex {
                    let connections = match data {
                        NewVertexData::Positioned(_, connections) => connections,
                        AutoPositioned(connections) => connections
                    };

                    // let is_connection = connections
                    //     .iter()
                    //     .any(|other_vertex| Rc::ptr_eq(vertex, other_vertex));
                    //
                    // if is_connection {
                    //     color = "green";
                    // }
                }

                html! {
                    <circle fill=color cx=screen_position.x.to_string() cy=screen_position.y.to_string() r=VERTEX_RADIUS.to_string()/>
                }
            }).collect::<Html>();

        let edges_html = self.graph_layout.graph
            .edge_indices()
            .enumerate()
            .map(|(index, edge_index)| {
                let (v_1_index, v_2_index) = self.graph_layout.graph.edge_endpoints(edge_index).unwrap();
                let v_1_pos = self.graph_layout.graph.node_weight(v_1_index).unwrap().position + self.origin;
                let v_2_pos = self.graph_layout.graph.node_weight(v_2_index).unwrap().position + self.origin;

                html! {
                    <>
                    <path id=format!("edge-{}-path", index) fill="none" stroke="black" d=format!("M{},{}L{},{}", v_1_pos.x, v_1_pos.y, v_2_pos.x, v_2_pos.y)/>
                    <text>
                        <textPath href=format!("#edge-{}-path", index) startOffset="50%" text-anchor="middle">
                            { format!("{:.00}", v_1_pos.distance(v_2_pos)) }
                        </textPath>
                    </text>
                    </>
                }
            }).collect::<Html>();

        let new_vertex_html = if let Some(NewVertexData::Positioned(position, _)) = &self.new_vertex {
            let Point2 { x, y } = position + self.origin;
            html! {
                <circle fill="red" cx=x.to_string() cy=y.to_string() r=VERTEX_RADIUS.to_string()/>
            }
        } else {
            "".into()
        };

        let new_vertex_buttons_html = if self.new_vertex.is_some() {
            html! {
                <>
                <button
                        type="button"
                        class="btn btn-primary align-self-start me-2"
                        onclick=self.link.callback(|_| Message::ConfirmNewVertex)
                >{ "Confirm" }</button>
                <button
                    type="button"
                    class="btn btn-danger align-self-start"
                    onclick=self.link.callback(|_| Message::CancelNewVertex)
                >{ "Cancel" }</button>
                </>
            }
        } else {
            "".into()
        };

        let mouse_down_callback = self.link.callback(|event: MouseEvent|
            Message::MouseDown(Point2::new(event.client_x(), event.client_y()))
        );

        let mouse_up_callback = self.link.callback(|_| Message::MouseUp);

        let mouse_move_callback = self.link.callback(|event: MouseEvent|
            Message::MouseMove(Point2::new(event.client_x(), event.client_y()))
        );

        let touch_start_callback = self.link.callback(|event: TouchEvent| {
            let touch = event.touches().get(0).unwrap();
            Message::MouseDown(Point2::new(touch.client_x(), touch.client_y()))
        });

        let touch_move_callback = self.link.callback(|event: TouchEvent| {
            let touch = event.touches().get(0).unwrap();
            Message::MouseMove(Point2::new(touch.client_x(), touch.client_y()))
        });

        let touch_end_callback = self.link.callback(|_| Message::MouseUp);

        let displacement_graph_path_data = path_data(&self.average_magnitudes, |(s, ..)| s, 1.);
        let velocity_graph_path_data = path_data(&self.average_magnitudes, |(_, v, _)| v, 5.);
        let acceleration_graph_path_data = path_data(&self.average_magnitudes, |(.., a)| a, 5.);

        let (s_mag, v_mag, a_mag) = self.average_magnitudes
            .front()
            .copied()
            .unwrap_or((0., 0., 0.));

        html! {
            <>
            <div class="position-fixed top-0 w-100 d-flex p-2" style="pointer-events: none">
                <div style="pointer-events: auto">
                    <button
                        type="button"
                        class="btn btn-secondary align-self-start me-2"
                        onclick=self.link.callback(|_| Message::StartAutoPosition)
                    >{ "Auto-place" }</button>
                    { new_vertex_buttons_html }
                </div>
                <div class="ms-auto w-25" style="pointer-events: auto">
                    <h2>
                        { format!("s̅: {:.02}, v̅: {:.02}, a̅: {:.02}", s_mag, v_mag, a_mag) }
                    </h2>
                    <h2>
                        { if self.settled {"Settled"} else {"Unsettled"} }
                    </h2>
                    <label class="form-label">{ format!("Target separation: {}", self.graph_layout.target_separation) }</label>
                    <input
                        type="range"
                        class="form-range"
                        value=self.graph_layout.target_separation.to_string()
                        min="10"
                        max="500"
                        oninput=param_changer_callback!(self, target_separation)
                    />
                    <label class="form-label">{ format!("Centering strength: {}", self.graph_layout.centering_strength) }</label>
                    <input
                        type="range"
                        class="form-range"
                        value=self.graph_layout.centering_strength.to_string()
                        min="0"
                        max="1"
                        step="0.01"
                        oninput=param_changer_callback!(self, centering_strength)
                    />
                    <label class="form-label">{ format!("Step scale: {}", self.graph_layout.step_scale) }</label>
                    <input
                        type="range"
                        class="form-range"
                        value=self.graph_layout.step_scale.to_string()
                        min="0"
                        max="1"
                        step="0.01"
                        oninput=param_changer_callback!(self, step_scale)
                    />
                    <label class="form-label">{ format!("Dampening factor: {}", self.graph_layout.dampening_factor) }</label>
                    <input
                        type="range"
                        class="form-range"
                        value=self.graph_layout.dampening_factor.to_string()
                        min="1"
                        max="5"
                        step="0.01"
                        oninput=param_changer_callback!(self, dampening_factor)
                    />
                </div>
            </div>
            <svg
                ref=self.svg_ref.clone()
                onmousedown=mouse_down_callback
                onmouseup=mouse_up_callback
                onmousemove=mouse_move_callback
                ontouchstart=touch_start_callback
                ontouchmove=touch_move_callback
                ontouchend=touch_end_callback
                ontouchcancel=touch_end_callback.clone()
                style="width: 100vw; height: 100vh; user-select: none"
            >
                { vertices_html }
                { edges_html }
                { new_vertex_html }
                <g transform=format!("translate({} {})", self.size.x * 0.75, self.size.y - 250.)>
                    <path stroke="black" fill="none" d=format!("M0 0{}", displacement_graph_path_data)/>
                </g>
                <g transform=format!("translate({} {})", self.size.x * 0.75, self.size.y - 150.)>
                    <path stroke="black" fill="none" d=format!("M0 0{}", velocity_graph_path_data)/>
                </g>
                <g transform=format!("translate({} {})", self.size.x * 0.75, self.size.y - 50.)>
                    <path stroke="black" fill="none" d=format!("M0 0{}", acceleration_graph_path_data)/>
                </g>
            </svg>
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.size = self.svg_ref
                .cast::<Element>()
                .map_or(Vector2::new(0., 0.), |svg_element|
                    Vector2::new(
                        svg_element.client_width() as f32,
                        svg_element.client_height() as f32,
                    ),
                );

            self.origin = self.size / 2.;
        }
    }
}

fn path_data(magnitudes: &VecDeque<(f32, f32, f32)>, extractor: impl Fn(&(f32, f32, f32)) -> &f32, y_scale: f32) -> String {
    magnitudes.iter()
        .enumerate()
        .filter_map(|(index, magnitudes)| {
            let value = extractor(magnitudes);
            (!value.is_nan()).then(|| format!("L{:.00} {:.00}", index * 2, -*value * y_scale))
        }).join("")
        .replacen("L", "M", 1)
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<App>();
    Ok(())
}