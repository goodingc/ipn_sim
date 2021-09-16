use cgmath::Point3;

use crate::movement::Movement;
use crate::utils::{SpaceMetric, TimeMetric};

type PathPosition = (TimeMetric, Point3<SpaceMetric>);

#[derive(Clone)]
pub struct PathMovement {
    positions: Vec<PathPosition>,
}

impl PathMovement {
    pub fn new(positions: Vec<PathPosition>) -> Self {
        PathMovement { positions }
    }
}

impl Movement for PathMovement {
    fn get_position_at(&self, time: u64) -> Point3<SpaceMetric> {
        for (index, position) in self.positions.iter().enumerate() {
            if position.0 == time {
                return position.1;
            }
            if position.0 > time {
                let prev_position = &self.positions[index - 1];
                let inter_time =
                    (time - prev_position.0) as SpaceMetric / (position.0 - prev_position.0) as SpaceMetric;
                return prev_position.1 + ((position.1 - prev_position.1) * inter_time);
            }
        }
        self.positions.last().unwrap().1
    }
}
