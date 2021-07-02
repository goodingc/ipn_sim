use ipn_sim_lib::ipn_sim::IpnSim;
use std::rc::Rc;
use std::cell::RefCell;
use yew::prelude::*;
use crate::bindings;
use yew::services::IntervalService;
use std::time::Duration;
use yew::services::interval::IntervalTask;

pub struct App {
    link: ComponentLink<Self>,
    tick_task: Option<IntervalTask>,
    sim_wrapper: Rc<RefCell<IpnSim>>,
}

pub enum AppMessage {
    SimTick
}

impl App {
    fn sim_setup(&mut self) {
        bindings::setup(
            // self.sim_wrapper
            //     .borrow_mut()
            //     .setup()
            //     .unchecked_into()
        );
    }
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            tick_task: None,
            sim_wrapper: Rc::new(RefCell::new(IpnSim::new(1000, vec![]))),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMessage::SimTick => {
                self.sim_wrapper.borrow_mut().tick();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="vw-100 vh-100" id="renderer-wrapper"/>
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