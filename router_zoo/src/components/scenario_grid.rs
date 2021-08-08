use std::iter;
use std::rc::Rc;
use std::time::Duration;

use web_sys::Element;
use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, RenderService};
use yew::services::interval::IntervalTask;
use yew::services::render::RenderTask;

use ipn_sim_lib::router::Router;
use ipn_sim_lib::routers::epidemic::epidemic::Epidemic;
use ipn_sim_lib::routers::epidemic::flavours::ack::Ack;
use ipn_sim_lib::utils::log;

use crate::router_details::{RouterDetails, RouterParamType};
use crate::scenario_grid_wrapper::ScenarioGridWrapper;
use crate::components::router_params_editor::RouterParamsEditor;

const GRAPH_ASPECT_RATIO: f32 = 16. / 9.;

pub struct ScenarioGrid<R: Router + RouterDetails + Clone + 'static> {
    link: Rc<ComponentLink<Self>>,
    wrapper: ScenarioGridWrapper,
    render_task: Option<RenderTask>,
    graph_name_header_element: Option<Element>,
}

impl<R: Router + RouterDetails + Clone + 'static> ScenarioGrid<R> {
    fn render(&mut self) {
        self.render_task = Some(RenderService::request_animation_frame(
            self.link.callback(|timestamp| ScenarioGridMessage::Tick(timestamp / 1000.))
        ));
    }
}

pub enum ScenarioGridMessage<R: Router + RouterDetails + Clone + 'static> {
    Tick(f64),
    SetRouter(R),
}

impl<R: Router + RouterDetails + Clone + 'static> Component for ScenarioGrid<R> {
    type Message = ScenarioGridMessage<R>;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: Rc::new(link),
            wrapper: ScenarioGridWrapper::new(Box::new(R::build_instance(R::default_params()))),
            render_task: None,
            graph_name_header_element: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Tick(timestamp) => {
                if self.wrapper.tick(1. / 30.) {
                    self.render_task = None;
                } else {
                    self.render();
                }
                true
            }
            Self::Message::SetRouter(router) => {
                self.wrapper = ScenarioGridWrapper::new(Box::new(router));
                if self.render_task.is_none() {
                    self.render();
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let header_html = self.wrapper.scenarios
            .iter()
            .map(|wrapper| html! {
                <th scope="col">{ wrapper.name.as_str() }</th>
            })
            .collect::<Html>();

        let graph_width = (794 - self.graph_name_header_element.as_ref().map_or(
            0,
            |element| {
                element.client_width() as u16
            },
        )) / self.wrapper.scenarios.len() as u16 - 16;
        let graph_height = (graph_width as f32 / GRAPH_ASPECT_RATIO) as u16;

        let body_html = (0..self.wrapper.scenarios[0].reports.len())
            .map(|report_index| {
                let cells_content = self.wrapper.scenarios
                    .iter()
                    .enumerate()
                    .map(|(graph_index, wrapper)| {
                        let graph_html = wrapper.reports[report_index]
                            .borrow()
                            .render_graph(
                                graph_width,
                                graph_height,
                                wrapper.sim.borrow().time,
                            );
                        html! {
                            <td key=format!("scenario-grid-cell-{}-{}", report_index, graph_index)>
                                {graph_html}
                            </td>
                        }
                    }).collect::<Html>();
                html! {
                    <tr>
                        <th scope="row">{"Graph"}</th>
                        {cells_content}
                    </tr>
                }
            }).collect::<Html>();

        html! {
            <div class="d-flex" style="width: calc(var(--body-width) + var(--gutter-width))">
                <table class="table m-0" style="width: var(--body-width)">
                    <thead>
                        <tr>
                            <th id="graph-name-header" scope="col">
                                {"Graph"}
                            </th>
                            {header_html}
                        </tr>
                    </thead>
                    <tbody>
                        {body_html}
                    </tbody>
                </table>
                <RouterParamsEditor<R> on_update=&self.link.callback(|router| Self::Message::SetRouter(router))/>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.render();
            self.graph_name_header_element = Some(get_element_by_id("graph-name-header"));
        }
    }
}

fn get_element_by_id(id: &str) -> Element {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap()
}