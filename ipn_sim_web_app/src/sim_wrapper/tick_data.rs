use ipn_sim_lib::node::Node;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use typescript_definitions::TypescriptDefinition;
use ipn_sim_lib::utils::TimeMetric;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Serialize, TypescriptDefinition)]
pub struct TickData<'a> {
    #[serde(rename="finalTick")]
    pub final_tick: bool,
    pub time: TimeMetric,
    pub nodes: &'a Vec<Rc<RefCell<Node>>>
}
