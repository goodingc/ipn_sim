use cgmath::Point3;
use serde::Serialize;
use typescript_definitions::TypescriptDefinition;
use wasm_bindgen::prelude::*;

use crate::movement::Movement;
use crate::utils::{SpaceMetric, TimeMetric};

#[derive(Serialize, TypescriptDefinition)]
pub struct Body {
    pub name: String,
    pub mass: f64,
    pub position: Point3<SpaceMetric>,
    #[serde(skip)]
    pub movement: Box<dyn Movement>,
    pub radius: SpaceMetric,
}

impl Body {
    pub fn set_position(&mut self, time: TimeMetric) {
        self.position = self.movement.get_position_at(time);
    }
}
