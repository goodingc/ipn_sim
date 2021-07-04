use yew::prelude::*;
use crate::sim_wrapper::sim_wrapper::SimWrapper;
use std::rc::Rc;
use std::cell::RefCell;
use ipn_sim_lib::node::Node;
use crate::utils;
use crate::components::data::Data;

pub struct NodesTab {
    link: ComponentLink<Self>,
    props: NodesTabProps,
    selected_node: Rc<RefCell<Node>>,
}

pub enum NodesComponentMessage {
    SetSelectedNode(usize)
}

#[derive(Properties, Clone)]
pub struct NodesTabProps {
    pub wrapper: Rc<RefCell<SimWrapper>>,
}

impl Component for NodesTab {
    type Message = NodesComponentMessage;
    type Properties = NodesTabProps;

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
                NodesComponentMessage::SetSelectedNode(element.selected_index() as usize)
            } else {
                unreachable!()
            }
        });

        let node_options = self.props.wrapper
            .borrow()
            .sim.nodes
            .as_ref()
            .unwrap()
            .iter()
            .map(|node| {
                let selected = Rc::ptr_eq(&self.selected_node, node);
                html! {
                    <option selected=selected>{ &node.borrow().name }</option>
                }
            }).collect::<Html>();

        let selected_node = self.selected_node.borrow();

        let message_buffer_html = selected_node.message_buffer.buffer
            .values()
            .map(|data| html!{
            <div class="row">
                <div class="col">
                    <h4 class="fw-normal">
                        <Data data=data.clone()/>
                    </h4>
                </div>
            </div>
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
            <div class="col">
                <h2>
                    { "Name:" }
                </h2>
            </div>
        </div>
        <div class="row">
            <div class="col">
                <h4 class="fw-normal">
                    { &selected_node.name }
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
        { message_buffer_html }
        </>
        }
    }
}