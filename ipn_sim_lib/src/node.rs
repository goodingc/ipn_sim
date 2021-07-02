use crate::movement::Movement;
use cgmath::Vector3;
use crate::utils::{SpaceMetric, TimeMetric};

pub struct Node {
    pub movement: Box<dyn Movement>,
    position: Vector3<SpaceMetric>,
}

impl Node {
    pub fn new(movement: impl Movement + 'static) -> Self {
        let movement = Box::new(movement);
        Self {
            position: movement.get_position_at(0),
            movement,
        }
    }

    pub fn set_position(&mut self, time: TimeMetric) {
        self.position = self.movement.get_position_at(time)
    }
}