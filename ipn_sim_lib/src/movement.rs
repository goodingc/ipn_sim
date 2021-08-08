use cgmath::Point3;
use downcast_rs::{impl_downcast, Downcast};
use dyn_clonable::clonable;

use crate::utils::{SpaceMetric, TimeMetric};
use std::hash::Hash;

#[clonable]
pub trait Movement: Clone + Downcast {
    fn get_position_at(&self, time: TimeMetric) -> Point3<SpaceMetric>;
}

impl_downcast!(Movement);

#[cfg(test)]
pub mod tests {
    use cgmath::Point3;

    use crate::movement::Movement;
    use crate::utils::{SpaceMetric, TimeMetric};

    pub fn test_movement(
        movement: impl Movement,
        expected_positions: impl Iterator<Item = (TimeMetric, Point3<SpaceMetric>)>,
    ) {
        expected_positions
            .for_each(|(time, position)| assert_eq!(movement.get_position_at(time), position));
    }
}
