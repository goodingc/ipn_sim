use std::iter;

use yew::prelude::*;

use crate::event_html::event_to_html;
use crate::utils::format_time::format_time;
use crate::utils::WrapperProps;

pub struct EventsTab {
    link: ComponentLink<Self>,
    props: WrapperProps,
    group_index: usize,
}

pub enum EventsTabMessage {
    SetGroup(usize),
}

impl Component for EventsTab {
    type Message = EventsTabMessage;
    type Properties = WrapperProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            group_index: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            EventsTabMessage::SetGroup(group_index) => {
                if group_index != self.group_index {
                    self.group_index = group_index;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let wrapper = self.props.wrapper.borrow();
        let event_log = &wrapper.web_app_report.borrow().event_log;
        let group_index = self.group_index;

        event_log
            .get_group(group_index)
            .map_or_else(
            || html! {
                {"No events"}
            },
            |(time, events)| {
                let prev_button_html = if group_index == 0 {
                    html! {
                        <button class="btn btn-secondary w-50" disabled=true>{"No previous events"}</button>
                    }
                } else {
                    html! {
                        <button
                            class="btn btn-primary w-50"
                            onclick=self.link.callback(move |_| EventsTabMessage::SetGroup(group_index - 1))
                        >{"-"}{format_time(time - event_log.get_group(group_index - 1).unwrap().0, None)}</button>
                    }
                };

                let next_button_html = if group_index == event_log.len().saturating_sub(1) {
                    html! {
                        <button class="btn btn-secondary w-50" disabled=true>{"No next events"}</button>
                    }
                } else {
                    html! {
                        <button
                            class="btn btn-primary w-50"
                            onclick=self.link.callback(move |_| EventsTabMessage::SetGroup(group_index + 1))
                        >{"+"}{format_time(event_log.get_group(group_index + 1).unwrap().0 - time, None)}</button>
                    }
                };


                let events_html = events
                    .iter()
                    .map(event_to_html)
                    .collect::<Html>();

                html! {
                    <>
                    <div class="row mb-2">
                        <div class="col">
                            <div class="btn-group w-100">
                                {prev_button_html}
                                {next_button_html}
                            </div>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            <h4 class="my-0 px-1">
                                <span>{ format_time(*time, None) }</span>
                            </h4>
                            <hr class="my-0" />
                        </div>
                    </div>
                    <div class="row" style="height: 800px; overflow-y: auto;">
                        <div class="col">
                            {events_html}
                        </div>
                    </div>
                    </>
                }
            },
        )

        // let row_count = self.props.wrapper
        //     .borrow()
        //     .report_data
        //     .borrow()
        //     .event_log
        //     .iter()
        //     .fold(
        //         0,
        //         |row_count, (_, events)|
        //             row_count + 1 + events.len(),
        //     );
        //
        //
        // let mut rows = vec![];
        // let mut rows_to_skip = (self.page_index * PAGE_ROWS) as isize;
        //
        // let wrapper = self.props.wrapper
        //     .borrow();
        // let event_log = &wrapper.report_data
        //     .borrow()
        //     .event_log;
        //
        // for (time, events) in event_log {
        //     let group_rows = events.len() + 1;
        //     rows_to_skip -= group_rows as isize;
        //     if rows_to_skip < 0 {
        //         rows.push(html! {
        //             <div class="row">
        //                 <div class="col">
        //                     <h4 class="my-0 px-1">
        //                         <span>{ format_time(*time) }</span>
        //                     </h4>
        //                     <hr class="my-0" />
        //                 </div>
        //             </div>
        //         });
        //         ipn_sim_lib::utils::log(&*format!("{}, {}, {}", rows_to_skip, (group_rows as isize).saturating_add(rows_to_skip).max(0) as usize, PAGE_ROWS));
        //         rows.append(
        //             &mut events
        //                 .iter()
        //                 .skip((group_rows as isize).saturating_add(rows_to_skip).max(0) as usize)
        //                 .take(PAGE_ROWS)
        //                 .map(event_to_html)
        //                 .collect()
        //         );
        //         if rows_to_skip < -(PAGE_ROWS as isize) {
        //             break;
        //         }
        //     }
        // }
        //
        // let rows_html = rows.into_iter().collect::<Html>();
        //
        // // let rows_html = self.props.wrapper
        // //     .borrow()
        // //     .report_data
        // //     .borrow()
        // //     .event_log
        // //     .iter()
        // //     .flat_map(|(time, events)|
        // //         iter::once_with(move || html! {
        // //             <div class="row">
        // //                 <div class="col">
        // //                     <h4 class="my-0 px-1">
        // //                         <span>{ format_time(*time) }</span>
        // //                     </h4>
        // //                     <hr class="my-0" />
        // //                 </div>
        // //             </div>
        // //         }).chain(
        // //             events
        // //                 .iter()
        // //                 .map(event_to_html)
        // //         )
        // //     ).skip(self.page_index * PAGE_ROWS)
        // //     .take(PAGE_ROWS)
        // //     .collect::<Html>();
        //
        // let buttons_html = vec![
        //     0,
        //     self.page_index.saturating_sub(5),
        //     self.page_index.saturating_sub(1),
        //     self.page_index,
        //     self.page_index.saturating_add(1),
        //     self.page_index.saturating_add(5),
        //     row_count / PAGE_ROWS + 1
        // ].into_iter()
        //     .map(|page_index| html! {
        //         <button
        //             class="btn btn-primary"
        //             onclick=self.link.callback(move |_| EventsTabMessage::SetPage(page_index))
        //         >{page_index}</button>
        //     })
        //     .collect::<Html>();
        //
        // html! {
        //     <>
        //     <div class="row mb-2">
        //         <div class="col">
        //             <div class="btn-group">
        //                 {buttons_html}
        //             </div>
        //         </div>
        //     </div>
        //     {rows_html}
        //     </>
        // }
    }
}
