use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use collision::{Continuous, Discrete, Ray, Sphere};
use wasm_bindgen::JsValue;

use ipn_sim_lib::cgmath::{InnerSpace, MetricSpace, Point3, Vector3};
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;
use ipn_sim_lib::report::Report;
use ipn_sim_lib::utils::{log, Shared, shared, SpaceMetric, TimeMetric};
use ipn_sim_reports::reports::{
    message_buffer_occupancy::MessageBufferOccupancy,
    message_states::MessageStates,
    send_deliver_ratio::SendDeliverRatio,
    message_flight_time::MessageFlightTime,
    messages::Messages
};

use crate::bindings;
use crate::sim_wrapper::interval_event::IntervalEvent;
use crate::sim_wrapper::setup_data::SetupData;
use crate::sim_wrapper::tick_data::TickData;
use crate::sim_wrapper::value_logger::ValueLogger;
use crate::sim_wrapper::web_app_report::{WebAppReport};
use ipn_sim_reports::reports::single_message_graph::SingleMessageGraph;

pub struct SimWrapper {
    pub sim: IpnSim,
    interval: TimeMetric,
    pub web_app_report: Shared<WebAppReport>,
    pub message_states_report: Shared<MessageStates>,
    pub message_buffer_occupancy_report: Shared<MessageBufferOccupancy>,
    pub send_deliver_ratio_report: Shared<SendDeliverRatio>,
    pub message_flight_time_report: Shared<MessageFlightTime>,
    pub messages_report: Shared<Messages>,
    pub single_message_graph_report: Shared<SingleMessageGraph>,
    pub highlighted_node_index: Option<usize>
}

impl SimWrapper {
    pub fn new(mut sim_builder: IpnSimBuilder, interval: TimeMetric) -> Self {
        let message_states_report = shared(MessageStates::new());
        let message_buffer_occupancy_report = shared(MessageBufferOccupancy::new());
        let send_deliver_ratio_report = shared(SendDeliverRatio::new());
        let message_flight_time_report = shared(MessageFlightTime::new());
        let messages_report = shared(Messages::default());
        let web_app_report = shared(WebAppReport::new());
        let single_message_graph_report = shared(SingleMessageGraph::default());
        let mut sim = sim_builder
            .add_event(0, IntervalEvent(interval))
            .add_shared_report(&web_app_report)
            .add_shared_report(&message_states_report)
            .add_shared_report(&message_buffer_occupancy_report)
            .add_shared_report(&send_deliver_ratio_report)
            .add_shared_report(&message_flight_time_report)
            .add_shared_report(&messages_report)
            .add_shared_report(&single_message_graph_report)
            .build();
        sim.init();
        Self {
            sim,
            interval,
            web_app_report,
            message_states_report,
            message_buffer_occupancy_report,
            send_deliver_ratio_report,
            message_flight_time_report,
            messages_report,
            single_message_graph_report,
            highlighted_node_index: None
        }
    }

    pub fn get_setup_data(&self) -> JsValue {
        JsValue::from_serde(&SetupData {
            nodes: self.sim.nodes.as_ref().unwrap(),
            bodies: &self.sim.bodies,
        }).unwrap()
    }

    pub fn tick(&mut self) -> JsValue {
        loop {
            let result = self.sim.tick();
            let time = self.sim.time;
            if time % self.interval == 0 {
                let nodes = self.sim.nodes.as_ref().unwrap();
                let connectable_node_indices = nodes
                    .iter()
                    .enumerate()
                    .flat_map(|(transmitting_node_index, transmitting_node)| {
                        let transmitting_node_ref = &*transmitting_node.borrow();
                        nodes
                            .iter()
                            .take(transmitting_node_index)
                            .enumerate()
                            .filter(|(_, receiving_node)| {
                                self.sim.nodes_can_transceive(
                                    transmitting_node_ref,
                                    &*receiving_node.borrow(),
                                )
                            })
                            .map(|(receiving_node_index, _)| {
                                (transmitting_node_index, receiving_node_index)
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();

                let mut web_app_report = self.web_app_report.borrow_mut();

                let sending_node_indices = web_app_report.sending_node_indices
                    .keys()
                    .cloned()
                    .collect();

                let camera_position = bindings::get_camera_position()
                    .into_serde::<Point3<SpaceMetric>>()
                    .unwrap();
                let occluded_node_indices = nodes
                    .iter()
                    .enumerate()
                    .filter_map(|(index, node)| {
                        let node_position = node.borrow().position;
                        let ray = Ray::new(
                            camera_position,
                            (node_position - camera_position).normalize(),
                        );
                        for body in &self.sim.bodies {
                            let body = body.borrow();
                            let collider = Sphere {
                                center: body.position,
                                radius: body.radius,
                            };
                            if let Some(intersection) = collider.intersection(&ray) {
                                if camera_position.distance(intersection)
                                    < camera_position.distance(node_position)
                                {
                                    return Some(index);
                                }
                            }
                        }
                        None
                    })
                    .collect();

                let data_js = JsValue::from_serde(&TickData {
                    final_tick: result.is_terminal(),
                    time: self.sim.time,
                    nodes,
                    connectable_node_indices,
                    bodies: &self.sim.bodies,
                    sending_node_indices,
                    creating_node_indices: mem::take(&mut web_app_report.creating_node_indices),
                    delivering_node_indices: mem::take(&mut web_app_report.delivering_node_indices),
                    message_buffer_occupancies: &self.message_buffer_occupancy_report.borrow().occupancies,
                    occluded_node_indices,
                    highlighted_node_index: &self.highlighted_node_index
                }).unwrap();

                web_app_report
                    .sending_node_indices
                    .retain(|_, sending| *sending);

                return data_js;
            }
        }
    }
}
