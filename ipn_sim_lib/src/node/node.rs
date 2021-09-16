use cgmath::Point3;
use serde::Serialize;
use typescript_definitions::TypescriptDefinition;
use wasm_bindgen::prelude::*;

use crate::node::message_buffer::MessageBuffer;
use crate::movement::Movement;
use crate::router::Router;
use crate::transceiver::transceiver::Transceiver;
use crate::utils::{NodeId, SpaceMetric, TimeMetric};

#[derive(Serialize, TypescriptDefinition)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    #[serde(skip)]
    pub movement: Box<dyn Movement>,
    pub position: Point3<SpaceMetric>,
    #[serde(skip)]
    pub message_buffer: MessageBuffer,
    #[serde(skip)]
    pub router: Option<Box<dyn Router>>,
    #[serde(skip)]
    pub transceiver: Transceiver,
}

impl Node {
    pub fn set_position(&mut self, time: TimeMetric) {
        self.position = self.movement.get_position_at(time)
    }
}
