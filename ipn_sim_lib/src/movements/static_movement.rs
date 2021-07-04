use cgmath::{Point3, Vector3};
use crate::utils::{SpaceMetric, TimeMetric};
use crate::movement::Movement;

#[derive(Clone)]
pub struct StaticMovement {
    point: Point3<SpaceMetric>,
}

impl StaticMovement {
    pub fn new(point: Point3<f64>) -> Self {
        Self { point }
    }
}

impl Movement for StaticMovement {
    fn get_position_at(&self, time: TimeMetric) -> Point3<SpaceMetric> {
        self.point
    }
}
