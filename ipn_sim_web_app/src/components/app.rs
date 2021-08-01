use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::IntervalService;

use crate::components::{
    events_tab::EventsTab,
    nodes_tab::NodesTab,
    router_log_tab::RouterLogTab,
    sidebar::{Sidebar, SidebarSide},
    stats_tab::StatsTab,
    tab_switcher::TabSwitcher,
};
use crate::sim_wrapper::sim_wrapper::SimWrapper;
use crate::{bindings, factories};
use ipn_sim_lib::utils::{Shared, shared};

pub struct App {
    link: ComponentLink<Self>,
    tick_task: Option<IntervalTask>,
    sim_wrapper: Shared<SimWrapper>,
}

pub enum AppMessage {
    SimTick,
}

impl App {
    fn sim_setup(&mut self) {
        bindings::setup(self.sim_wrapper.borrow_mut().get_setup_data());
    }
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            tick_task: None,
            sim_wrapper: shared(SimWrapper::new(
                factories::orbiting_rings(),
                1_000_000_000 * 5,
            )),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMessage::SimTick => {
                bindings::tick(self.sim_wrapper.borrow_mut().tick());
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let left_tabs = Rc::new(vec![
            (
                "Routers".to_string(),
                html! {
                    <StatsTab wrapper=&self.sim_wrapper></StatsTab>
                },
            ),
            (
                "Nodes".to_string(),
                html! {
                    <NodesTab wrapper=&self.sim_wrapper></NodesTab>
                },
            ),
        ]);

        let right_tabs = Rc::new(vec![
            (
                "Events".to_string(),
                html! {
                    <EventsTab wrapper=&self.sim_wrapper></EventsTab>
                },
            ),
            (
                "Router Log".to_string(),
                html! {
                    <RouterLogTab wrapper=&self.sim_wrapper></RouterLogTab>
                },
            ),
        ]);

        html! {
            <>
            <div class="vw-100 vh-100" id="renderer-wrapper"/>
            <Sidebar side=SidebarSide::Left title="Left">
                <TabSwitcher tabs=&left_tabs></TabSwitcher>
            </Sidebar>
            <Sidebar side=SidebarSide::Right title="Right">
                <TabSwitcher tabs=&right_tabs></TabSwitcher>
            </Sidebar>
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.sim_setup();
            self.tick_task = Some(IntervalService::spawn(
                Duration::from_secs_f64(1. / 30.),
                self.link.callback(|_| AppMessage::SimTick),
            ))
        }
    }
}
