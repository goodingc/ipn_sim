use std::iter;

use yew::prelude::*;

use event_html::EventHtml;
use ipn_sim_lib::event::Event;
use ipn_sim_lib::events::awake_router_event::AwakeRouterEvent;
use ipn_sim_lib::events::create_message_event::CreateMessageEvent;
use ipn_sim_lib::events::receive_data_event::ReceiveDataEvent;
use ipn_sim_lib::events::router_event::{RouterEvent, RouterEventType};
use ipn_sim_lib::events::transmit_end_event::TransmitEndEvent;
use ipn_sim_lib::events::transmit_start_event::TransmitStartEvent;

use crate::components::data::Data;

pub mod event_html;
mod router_event;

pub fn event_to_html(event: &Box<dyn Event>) -> Html {
    CreateMessageEvent::try_to_html(event)
        .or_else(|| TransmitStartEvent::try_to_html(event))
        .or_else(|| TransmitEndEvent::try_to_html(event))
        .or_else(|| ReceiveDataEvent::try_to_html(event))
        .or_else(|| AwakeRouterEvent::try_to_html(event))
        .or_else(|| RouterEvent::try_to_html(event))
        .unwrap_or(html! {
        <div class="row border-bottom border-secondary">
            <div class="col">
                {"Oops"}
            </div>
        </div>
        })
}
