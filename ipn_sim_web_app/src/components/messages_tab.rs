use std::rc::Rc;

use yew::prelude::*;

use ipn_sim_lib::utils::MessageId;
use ipn_sim_reports::reports::messages::State;

use crate::utils::{format_time::format_time, message_destination_string, WrapperProps};
// use crate::components::message_graph::MessageGraph;
use ipn_sim_lib::node::node::Node;
use yew::services::ConsoleService;

pub struct MessagesTab {
    link: ComponentLink<Self>,
    props: WrapperProps,
    selected_message_id: Option<MessageId>,
}

pub enum MessagesTabMessage {
    SetSelectedMessage(MessageId)
}

impl Component for MessagesTab {
    type Message = MessagesTabMessage;
    type Properties = WrapperProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            selected_message_id: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MessagesTabMessage::SetSelectedMessage(id) => {
                self.selected_message_id = Some(id);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let message_options_html = self.props.wrapper
            .borrow()
            .messages_report
            .borrow()
            .messages
            .iter()
            .map(|(id, message)| {
                let selected = self.selected_message_id
                    .map_or(false, |selected_message_id| *id == selected_message_id);

                let class = match message.state {
                    State::InFlight => "bg-warning",
                    State::Delivered => "bg-success",
                    State::Dropped => "bg-danger"
                };

                html! {
                    <option class=class selected=selected value=id.to_string()>
                        { format!(
                            "ID: {}, created at: {}, TTL: {}",
                            id,
                            format_time(message.time_created, Some(2)),
                            message.ttl.map_or("none".to_string(), |ttl| format_time(ttl, Some(2)))
                        ) }
                    </option>
                }
            }).collect::<Html>();

        let content_html = self.selected_message_id
            .map_or_else(|| "No message".into(), |id| {

                let wrapper = self.props.wrapper
                    .borrow();

                let messages_report = wrapper.messages_report
                    .borrow();

                let message = messages_report.messages
                    .get(&id)
                    .unwrap();

                let (state_text, badge_variant) = match message.state {
                    State::InFlight => ("In Flight", "warning"),
                    State::Delivered => ("Delivered", "success"),
                    State::Dropped => ("Dropped", "danger")
                };

                html! {
                    <>
                    <div class="row">
                        <div class="col-4">
                            <h2>
                                { "ID:" }
                            </h2>
                        </div>
                        <div class="col-4">
                            <h2>
                                { "Source:" }
                            </h2>
                        </div>
                        <div class="col-4">
                            <h2>
                                { "State:" }
                            </h2>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col-4">
                            <h4 class="fw-normal">
                                { id }
                            </h4>
                        </div>
                        <div class="col-4">
                            <h4 class="fw-normal">
                                { &message.source.borrow().name }
                            </h4>
                        </div>
                        <div class="col-4">
                            <h4 class="fw-normal">
                                <span class=format!("badge bg-{}", badge_variant)>{ state_text }</span>
                            </h4>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            <h2>
                                { "Destination:" }
                            </h2>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            <h4 class="fw-normal">
                                { message_destination_string(&message.destination) }
                            </h4>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col-6">
                            <h2>
                                { "Created at:" }
                            </h2>
                        </div>
                        <div class="col-6">
                            <h2>
                                { "TTL:" }
                            </h2>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col-6">
                            <h4 class="fw-normal">
                                { format_time(message.time_created, Some(3)) }
                            </h4>
                        </div>
                        <div class="col-6">
                            <h4 class="fw-normal">
                                { message.ttl.map_or("none".to_string(), |ttl| format_time(ttl, Some(3))) }
                            </h4>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col-6">
                            <h2>
                                { "Copies:" }
                            </h2>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col-6">
                            <h4 class="fw-normal">
                                { message.copies }
                            </h4>
                        </div>
                    </div>
                    // <div class="row">
                    //     <div class="col">
                    //         <MessageGraph wrapper=&self.props.wrapper message_id=id/>
                    //     </div>
                    // </div>
                    </>
                }
            });

        let on_change_callback = self.link.callback(|data: ChangeData| {
            if let ChangeData::Select(element) = data {
                Self::Message::SetSelectedMessage(element.value().parse().unwrap())
            } else {
                unreachable!()
            }
        });

        html! {
            <>
            <div class="row mb-2">
                <div class="col">
                    <select class="form-select" onchange=on_change_callback >
                        { message_options_html }
                    </select>
                </div>
            </div>
            { content_html }
            </>
        }
    }
}