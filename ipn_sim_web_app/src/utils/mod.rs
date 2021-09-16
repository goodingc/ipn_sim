use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

use ipn_sim_lib::cgmath::Point3;
use ipn_sim_lib::utils::{Shared, SpaceMetric};

use crate::sim_wrapper::sim_wrapper::SimWrapper;
use ipn_sim_lib::message_destination::MessageDestination;
use ipn_sim_lib::node::node::Node;

pub mod format_time;

pub fn format_position(position: Point3<SpaceMetric>) -> String {
    format!(
        "({:.2}km, {:.2}km, {:.2}km)",
        position.x / 1000.,
        position.y / 1000.,
        position.z / 1000.,
    )
}

#[derive(Properties, Clone)]
pub struct WrapperProps {
    pub wrapper: Shared<SimWrapper>,
}

pub fn message_destination_string(destination: &MessageDestination<Shared<Node>>) -> String {
    match destination {
        MessageDestination::<Shared<Node>>::All => String::from("all nodes"),
        MessageDestination::<Shared<Node>>::Single(node) => node.borrow().name.clone(),
        MessageDestination::<Shared<Node>>::Multiple(nodes) => nodes
            .iter()
            .map(|node| node.borrow().name.clone())
            .collect::<Vec<_>>()
            .join(", "),
    }
}
