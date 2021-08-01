use crate::movement::Movement;
use crate::utils::{SpaceMetric, TimeMetric};
use cgmath::Point3;

#[derive(Clone)]
pub struct StaticMovement {
    pub point: Point3<SpaceMetric>,
}

impl StaticMovement {
    pub fn new(point: Point3<f64>) -> Self {
        Self { point }
    }
}

impl Movement for StaticMovement {
    fn get_position_at(&self, _time: TimeMetric) -> Point3<SpaceMetric> {
        self.point
    }
}
