use ipn_sim_lib::node::Node;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use typescript_definitions::TypescriptDefinition;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Serialize, TypescriptDefinition)]
pub struct SetupData<'a> {
    pub nodes: &'a Vec<Rc<RefCell<Node>>>
}
