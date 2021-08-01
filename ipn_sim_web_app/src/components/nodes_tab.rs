use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

use ipn_sim_lib::node::Node;

use crate::components::data::Data;
use crate::event_html::event_html::EventHtml;
use crate::sim_wrapper::sim_wrapper::SimWrapper;
use crate::utils;
use crate::utils::{format_time::format_time, WrapperProps};
use ipn_sim_lib::utils::{NodeId, Shared};
use std::iter;

pub struct NodesTab {
    link: ComponentLink<Self>,
    props: WrapperProps,
    selected_node: Shared<Node>,
}

pub enum NodesComponentMessage {
    SetSelectedNode(NodeId),
}

impl Component for NodesTab {
    type Message = NodesComponentMessage;
    type Properties = WrapperProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let selected_node = props.wrapper.borrow_mut().sim.get_node(0);
        Self {
            link,
            props,
            selected_node,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NodesComponentMessage::SetSelectedNode(index) => {
                self.selected_node = self.props.wrapper.borrow_mut().sim.get_node(index);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let on_change_callback = self.link.callback(|data: ChangeData| {
            if let ChangeData::Select(element) = data {
                NodesComponentMessage::SetSelectedNode(element.selected_index() as NodeId)
            } else {
                unreachable!()
            }
        });

        let node_options = self
            .props
            .wrapper
            .borrow()
            .sim
            .nodes
            .as_ref()
            .unwrap()
            .iter()
            .map(|node| {
                let selected = Rc::ptr_eq(&self.selected_node, node);
                html! {
                    <option selected=selected>{ &node.borrow().name }</option>
                }
            })
            .collect::<Html>();

        let selected_node = self.selected_node.borrow();
        let message_buffer = &selected_node.message_buffer;

        let message_buffer_html = message_buffer
            .buffer
            .values()
            .map(|data| {
                html! {
                <div class="row">
                    <div class="col">
                        <h4 class="fw-normal">
                            <Data data=data.clone()/>
                        </h4>
                    </div>
                </div>
                }
            })
            .collect::<Html>();

        let router_log_html = self.props.wrapper
            .borrow()
            .web_app_report
            .borrow()
            .router_log
            .iter()
            .filter_map(|(time, events)| {
                let relevant_events = events
                    .into_iter()
                    .filter(|event| {
                        &*event.node.borrow() as *const Node == &*selected_node as *const Node
                    })
                    .collect::<Vec<_>>();

                if relevant_events.is_empty() {
                    None
                } else {
                    Some((time, relevant_events))
                }
            })
            .map(|(time, events)| {
                let events_html = events
                    .iter()
                    .map(|event| {
                        let details_html = event
                            .get_details()
                            .into_iter()
                            .map(|html| {
                                html! {
                                    <span class="me-2">
                                        { html }
                                    </span>
                                }
                            })
                            .collect::<Html>();
                        html! {
                            <div class="row border-bottom border-secondary">
                                <div class="col">
                                    { details_html }
                                </div>
                            </div>
                        }
                    })
                    .collect::<Html>();
                html! {
                    <>
                    <div class="row">
                        <div class="col">
                            <h4 class="my-0 px-1">
                                <span>{ format_time(*time, None) }</span>
                            </h4>
                            <hr class="my-0" />
                        </div>
                    </div>
                    { events_html }
                    </>
                }
            })
            .collect::<Html>();

        html! {
        <>
        <div class="row mb-2">
            <div class="col">
                <select class="form-select" onchange=on_change_callback>
                    { node_options }
                </select>
            </div>
        </div>
        <div class="row">
            <div class="col-6">
                <h2>
                    { "Name:" }
                </h2>
            </div>
            <div class="col-6">
                <h2>
                    { "ID:" }
                </h2>
            </div>
        </div>
        <div class="row">
            <div class="col-6">
                <h4 class="fw-normal">
                    { &selected_node.name }
                </h4>
            </div>
            <div class="col-6">
                <h4 class="fw-normal">
                    { selected_node.id }
                </h4>
            </div>
        </div>
        <div class="row">
            <div class="col">
                <h2>
                    { "Position:" }
                </h2>
            </div>
        </div>
        <div class="row">
            <div class="col">
                <h4 class="fw-normal">
                    { utils::format_position(selected_node.position) }
                </h4>
            </div>
        </div>
        <div class="row">
            <div class="col">
                <h2>
                    { "Message Buffer:" }
                </h2>
            </div>
        </div>
        <div class="row">
            <div class="col">
                {message_buffer.size}{"b / "}
                {message_buffer.capacity}{"b ("}
                {message_buffer.get_occupancy() * 100.}{"%)"}
            </div>
        </div>
        { message_buffer_html }
        <div class="row">
            <div class="col">
                <h2>
                    { "Router Log:" }
                </h2>
            </div>
        </div>
        <div class="row" style="height: 600px; overflow-y: auto;">
            <div class="col">
                { router_log_html }
            </div>
        </div>
        </>
        }
    }
}
