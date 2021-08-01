use std::cell::RefCell;
use std::rc::Rc;

use serde::Serialize;
use typescript_definitions::TypescriptDefinition;
use wasm_bindgen::prelude::*;

use ipn_sim_lib::body::Body;
use ipn_sim_lib::cgmath::Point3;
use ipn_sim_lib::node::Node;
use ipn_sim_lib::utils::{SpaceMetric, Shared};

#[derive(Serialize, TypescriptDefinition)]
pub struct SetupData<'a> {
    pub nodes: &'a Vec<Shared<Node>>,
    pub bodies: &'a Vec<Shared<Body>>,
}
