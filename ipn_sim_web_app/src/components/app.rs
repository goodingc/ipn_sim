use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::IntervalService;

use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;

use crate::{bindings, factories};
use crate::components::{sidebar::{Sidebar, SidebarSide}, nodes_tab::NodesTab};
use crate::sim_wrapper::sim_wrapper::SimWrapper;

pub struct App {
    link: ComponentLink<Self>,
    tick_task: Option<IntervalTask>,
    sim_wrapper: Rc<RefCell<SimWrapper>>,
}

pub enum AppMessage {
    SimTick,
}

impl App {
    fn sim_setup(&mut self) {
        bindings::setup(
            JsValue::from_serde(
                &self.sim_wrapper
                    .borrow_mut()
                    .get_setup_data()
            ).unwrap().unchecked_into()
        );
    }
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            tick_task: None,
            sim_wrapper: Rc::new(RefCell::new(SimWrapper::wrap(factories::simple_scenario(), 1_000_000_000 * 60))),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMessage::SimTick => {
                bindings::tick(
                    JsValue::from_serde(
                        &self.sim_wrapper
                            .borrow_mut()
                            .tick()
                    ).unwrap().unchecked_into()
                );
                true
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <div class="vw-100 vh-100" id="renderer-wrapper"/>
            <Sidebar side=SidebarSide::Left title="Left">
                <NodesTab wrapper=&self.sim_wrapper></NodesTab>
            </Sidebar>
            <Sidebar side=SidebarSide::Right title="Right">
                {"World"}
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