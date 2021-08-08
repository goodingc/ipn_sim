use ipn_sim_lib::router::Router;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::utils::{Shared, shared, TimeMetric};
use ipn_sim_reports::graph_report::GraphReport;
use crate::scenario::Scenario;
use ipn_sim_reports::reports::message_states::MessageStates;
use ipn_sim_reports::reports::send_deliver_ratio::SendDeliverRatio;
use std::collections::{BTreeMap, BinaryHeap};
use std::iter::FromIterator;
use std::rc::Rc;
use ipn_sim_lib::downcast_rs::__std::cmp::Ordering;
use std::cmp::Reverse;
use ipn_sim_reports::reports::message_buffer_occupancy::MessageBufferOccupancy;
use ipn_sim_reports::reports::message_flight_time::MessageFlightTime;
use ipn_sim_reports::reports::node_positions::NodePositions;
use yew::prelude::*;

pub struct ScenarioGridWrapper {
    pub scenarios: Vec<SimWrapper>,
    tick_queue: BinaryHeap<Reverse<TickQueueEntry>>,
}

pub struct SimWrapper {
    pub name: String,
    pub sim: Shared<IpnSim>,
    pub reports: Vec<Shared<dyn GraphReport>>,
}

pub struct TickQueueEntry(TimeMetric, Shared<IpnSim>);

impl ScenarioGridWrapper {
    pub fn new(router: Box<dyn Router>) -> Self {
        let sims = Scenario::scenarios()
            .into_iter()
            .map(|scenario| {
                let node_positions_report = shared(NodePositions::new());
                let message_states_report = shared(MessageStates::new());
                let message_buffer_occupancy_report = shared(MessageBufferOccupancy::new());
                let message_flight_time_report = shared(MessageFlightTime::new());
                let send_deliver_ratio_report = shared(SendDeliverRatio::new());

                let mut sim = scenario.build(router.clone())
                    .add_shared_report(&node_positions_report)
                    .add_shared_report(&message_states_report)
                    .add_shared_report(&message_buffer_occupancy_report)
                    .add_shared_report(&message_flight_time_report)
                    .add_shared_report(&send_deliver_ratio_report)
                    .build();
                sim.init();

                SimWrapper {
                    name: scenario.name,
                    sim: shared(sim),
                    reports: vec![
                        node_positions_report,
                        message_states_report,
                        message_buffer_occupancy_report,
                        message_flight_time_report,
                        send_deliver_ratio_report
                    ],
                }
            }).collect::<Vec<_>>();
        Self {
            tick_queue: BinaryHeap::from_iter(
                sims
                    .iter()
                    .map(|wrapper| Reverse(TickQueueEntry(0, Rc::clone(&wrapper.sim))))
            ),
            scenarios: sims,
        }
    }

    pub fn tick(&mut self, tick_length: f64) -> bool {
        let end_time = get_time() + tick_length;
        while get_time() <= end_time {
            if let Some(Reverse(TickQueueEntry(_, next_sim))) = self.tick_queue.pop() {
                if !next_sim.borrow_mut().tick().is_terminal() {
                    let sim_time = next_sim.borrow().time;
                    self.tick_queue.push(Reverse(TickQueueEntry(sim_time, next_sim)))
                }
            } else {
                return true;
            }
        }
        false
    }
}

fn get_time() -> f64 {
    web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance")
        .now() / 1000.
}

impl Eq for TickQueueEntry {}

impl PartialEq<Self> for TickQueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd<Self> for TickQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for TickQueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}