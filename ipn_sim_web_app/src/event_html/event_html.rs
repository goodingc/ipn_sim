use yew::prelude::*;

use ipn_sim_lib::event::Event;
use ipn_sim_lib::events::awake_router_event::AwakeRouterEvent;
use ipn_sim_lib::events::create_message_event::CreateMessageEvent;
use ipn_sim_lib::events::receive_data_event::ReceiveDataEvent;
use ipn_sim_lib::events::transmit_end_event::TransmitEndEvent;
use ipn_sim_lib::events::transmit_start_event::TransmitStartEvent;

use crate::components::data::Data;

pub trait EventHtml: Event {
    fn get_title(&self) -> &'static str;

    fn get_details(&self) -> Vec<Html>;

    fn to_html(&self) -> Html {
        let details_html: Html = self
            .get_details()
            .into_iter()
            .map(|html| {
                html! {
                <span class="me-2">
                    { html }
                </span>
                }
            })
            .collect();
        html! {
        <div class="row border-bottom border-secondary">
            <div class="col">
                <span class="me-2 fw-bold">{ self.get_title() }</span> { details_html }
            </div>
        </div>
        }
    }

    fn try_to_html(event: &Box<dyn Event>) -> Option<Html>
    where
        Self: Sized,
    {
        event
            .downcast_ref::<Self>()
            .map(|event: &Self| event.to_html())
    }
}

impl EventHtml for CreateMessageEvent {
    fn get_title(&self) -> &'static str {
        "CreateMessage"
    }

    fn get_details(&self) -> Vec<Html> {
        vec![
            Html::from(&self.node.borrow().name),
            html! {<Data data=self.payload.clone()/>},
        ]
    }
}

impl EventHtml for TransmitStartEvent {
    fn get_title(&self) -> &'static str {
        "TransmitStart"
    }

    fn get_details(&self) -> Vec<Html> {
        vec![Html::from(&self.node.borrow().name)]
    }
}

impl EventHtml for TransmitEndEvent {
    fn get_title(&self) -> &'static str {
        "TransmitEnd"
    }

    fn get_details(&self) -> Vec<Html> {
        vec![
            Html::from(&self.node.borrow().name),
            html! {<Data data=self.data.clone()/>},
        ]
    }
}

impl EventHtml for ReceiveDataEvent {
    fn get_title(&self) -> &'static str {
        "ReceiveData"
    }

    fn get_details(&self) -> Vec<Html> {
        vec![
            Html::from(&self.node.borrow().name),
            html! {<Data data=self.data.clone()/>},
        ]
    }
}

impl EventHtml for AwakeRouterEvent {
    fn get_title(&self) -> &'static str {
        "AwakeRouter"
    }

    fn get_details(&self) -> Vec<Html> {
        vec![Html::from(&self.node.borrow().name)]
    }
}
