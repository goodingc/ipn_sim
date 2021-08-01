use std::cell::RefCell;
use std::rc::Rc;

use num_traits::Num;
use yew::prelude::*;

use ipn_sim_lib::utils::{Shared, TimeMetric};
use ipn_sim_reports::graph_report::GraphReport;

use crate::sim_wrapper::sim_wrapper::SimWrapper;
use crate::utils::format_time::format_time;
use crate::utils::WrapperProps;

const LEFT_MARGIN: u16 = 30;
const RIGHT_MARGIN: u16 = 5;
const TOP_MARGIN: u16 = 10;
const BOTTOM_MARGIN: u16 = 20;

const HEIGHT: u16 = 200;

const HORIZONTAL_TICK_SPACING: u16 = 30;
const VERTICAL_TICK_SPACING: u16 = 20;

pub struct Graph<G: GraphReport + Clone + 'static> {
    link: ComponentLink<Self>,
    props: Props<G>,
    width: Option<u16>,
    expanded: bool,
    id_suffix: usize,
}

#[derive(Properties, Clone)]
pub struct Props<G: GraphReport + Clone> {
    pub children: Children,
    pub sim_time: TimeMetric,
    pub graph_report: Shared<G>
}

impl<G: GraphReport + Clone + 'static> Component for Graph<G> {
    type Message = ();
    type Properties = Props<G>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            width: None,
            expanded: false,
            id_suffix: rand::random::<usize>(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
            <div class="card" data-bs-toggle="collapse" href=format!("#graph-report-wrapper-{}", self.id_suffix)>
                <div class="card-body p-2">
                    {self.props.children.clone()}
                </div>
            </div>
            <div class="collapse" id=format!("graph-report-wrapper-{}", self.id_suffix)>
                {self.props.graph_report.borrow().render_graph(self.width.unwrap_or(0), HEIGHT, self.props.sim_time) }
            </div>
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let width = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(&format!("graph-report-wrapper-{}", self.id_suffix))
            .unwrap()
            .client_width() as u16;
        let changed = if let Some(old_width) = self.width {
            width != old_width
        } else {
            true
        };
        if changed {
            self.width = Some(width);
        }
    }
}
