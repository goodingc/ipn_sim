use yew::prelude::*;

use ipn_sim_lib::utils::TimeMetric;
use ipn_sim_reports::{
    graph_report::GraphReport,
    reports::{
        message_buffer_occupancy::MessageBufferOccupancy,
        message_states::MessageStates,
        send_deliver_ratio::SendDeliverRatio,
        message_flight_time::MessageFlightTime,
    },
};

use crate::components::graph::Graph;
use crate::utils::{format_time::format_time, WrapperProps};

pub struct StatsTab {
    link: ComponentLink<Self>,
    props: WrapperProps,
}

impl Component for StatsTab {
    type Message = ();
    type Properties = WrapperProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let wrapper = self.props.wrapper.borrow();
        let web_app_report = wrapper.web_app_report.borrow();

        let message_states_report = wrapper.message_states_report.borrow();

        html! {
            <div class="row" style="height: calc(100% - 35px); overflow-y: auto">
                <div class="col">
                    <div class="row">
                        <div class="col">
                            <h2>
                                { "Messages" }
                            </h2>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            <Graph<MessageStates> graph_report=&wrapper.message_states_report sim_time=wrapper.sim.time>
                                <div class="row">
                                    <div class="col-4">
                                        <h5 class="fw-normal m-0">
                                            {"Created: "}
                                            <span class="badge bg-warning px-1">{ message_states_report.created_message_counts.value }</span>
                                        </h5>
                                    </div>
                                    <div class="col-4">
                                        <h5 class="fw-normal m-0">
                                            {"Delivered: "}
                                            <span class="badge bg-success px-1">{ message_states_report.delivered_message_counts.value }</span>
                                        </h5>
                                    </div>
                                    <div class="col-4">
                                        <h5 class="fw-normal m-0">
                                            {"Dropped: "}
                                            <span class="badge bg-danger px-1">{ message_states_report.dropped_message_counts.value }</span>
                                        </h5>
                                    </div>
                                </div>
                            </Graph<MessageStates>>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            <h2>
                                { "Message Buffer Occupancy" }
                            </h2>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            <Graph<MessageBufferOccupancy> graph_report=&wrapper.message_buffer_occupancy_report sim_time=wrapper.sim.time>
                                <div class="row">
                                    <div class="col">
                                        <h5 class="fw-normal m-0">
                                            {"Current average: "}
                                            {
                                                (wrapper
                                                    .message_buffer_occupancy_report
                                                    .borrow()
                                                    .average_message_buffer_occupancies
                                                    .value * 1000.).round() / 10.
                                            }
                                            {"%"}
                                        </h5>
                                    </div>
                                </div>
                            </Graph<MessageBufferOccupancy>>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            <h2>
                                { "Message Flight Time" }
                            </h2>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            <Graph<MessageFlightTime> graph_report=&wrapper.message_flight_time_report sim_time=wrapper.sim.time>
                                <div class="row">
                                    <div class="col">
                                        <h5 class="fw-normal m-0">
                                            {"Average: "}
                                            {format_time(
                                                wrapper
                                                    .message_flight_time_report
                                                    .borrow()
                                                    .average_message_flight_times
                                                    .value as TimeMetric,
                                                None
                                            )}
                                        </h5>
                                    </div>
                                </div>
                            </Graph<MessageFlightTime>>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            <h2>
                                { "Send / Deliver Ratio" }
                            </h2>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            <Graph<SendDeliverRatio> graph_report=&wrapper.send_deliver_ratio_report sim_time=wrapper.sim.time>
                            <div class="row">
                                <div class="col">
                                    <h5 class="fw-normal m-0">
                                        {"Currently: "}
                                        { wrapper.send_deliver_ratio_report.borrow().send_deliver_ratios.value }
                                    </h5>
                                </div>
                            </div>
                            </Graph<SendDeliverRatio>>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
