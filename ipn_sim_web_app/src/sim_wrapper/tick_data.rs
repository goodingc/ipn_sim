use std::cell::RefCell;
use std::rc::Rc;

use serde::Serialize;
use typescript_definitions::TypescriptDefinition;
use wasm_bindgen::prelude::*;

use ipn_sim_lib::body::Body;
use ipn_sim_lib::node::Node;
use ipn_sim_lib::utils::{NodeId, TimeMetric, Shared};

#[derive(Serialize, TypescriptDefinition)]
#[serde(rename_all = "camelCase")]
pub struct TickData<'a> {
    pub final_tick: bool,
    pub time: TimeMetric,
    pub nodes: &'a Vec<Shared<Node>>,
    pub connectable_node_indices: Vec<(usize, usize)>,
    pub bodies: &'a Vec<Shared<Body>>,
    pub sending_node_indices: Vec<(usize, usize)>,
    pub message_buffer_occupancies: &'a Vec<f32>,
    pub creating_node_indices: Vec<usize>,
    pub delivering_node_indices: Vec<usize>,
    pub occluded_node_indices: Vec<usize>,
}
