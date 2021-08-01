use std::iter;

use yew::prelude::*;

use crate::event_html::{event_html::EventHtml, event_to_html};
use crate::utils::{format_time::format_time, WrapperProps};

pub struct RouterLogTab {
    link: ComponentLink<Self>,
    props: WrapperProps,
    prev_log_len: usize,
}

impl Component for RouterLogTab {
    type Message = ();
    type Properties = WrapperProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            prev_log_len: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        let long_len = self.props.wrapper
            .borrow()
            .web_app_report
            .borrow()
            .router_log
            .len();
        if long_len != self.prev_log_len {
            self.prev_log_len = long_len;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let rows_html = self.props.wrapper
            .borrow()
            .web_app_report
            .borrow()
            .router_log
            .iter()
            .flat_map(|(time, events)| {
                iter::once_with(move || {
                    html! {
                        <div class="row">
                            <div class="col">
                                <h4 class="my-0 px-1">
                                    <span>{ format_time(*time, None) }</span>
                                </h4>
                                <hr class="my-0" />
                            </div>
                        </div>
                    }
                })
                .chain(events.iter().map(|event| {
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
                }))
            })
            .collect::<Html>();

        html! {
            <>
            <div class="row" style="height: 800px; overflow-y: auto;">
                <div class="col">
                    {rows_html}
                </div>
            </div>
            </>
        }
    }
}
