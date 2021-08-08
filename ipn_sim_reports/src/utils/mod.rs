use std::iter;
use std::iter::FromIterator;

use ipn_sim_lib::events::router_event::MessageDestination;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::utils::NodeId;

pub mod format_time;
pub mod paths;

pub fn mean_std_dev(values: &Vec<f32>) -> (f32, f32) {
    let mean = values.iter().sum::<f32>() / values.len() as f32;
    let std_dev = values
        .iter()
        .map(|value| (*value - mean).abs())
        .sum::<f32>()
        / values.len() as f32;

    (mean, std_dev)
}

pub fn destination_to_ids<T: FromIterator<NodeId>>(destination: &MessageDestination, sim: &IpnSim) -> T {
    match destination {
        MessageDestination::All =>
            sim.nodes.as_ref().unwrap().iter().map(|node| node.borrow().id).collect(),
        MessageDestination::Single(node) =>
            iter::once(node.borrow().id).collect(),
        MessageDestination::Multiple(nodes) =>
            nodes.iter().map(|node| node.borrow().id).collect()
    }
}
