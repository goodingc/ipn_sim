use cgmath::{Point3, Vector3};

use crate::utils::{SpaceMetric, TimeMetric};

pub trait Movement {
    fn get_position_at(&self, time: TimeMetric) -> Point3<SpaceMetric>;
}

#[cfg(test)]
pub mod tests {
    use cgmath::Point3;

    use crate::movement::Movement;
    use crate::utils::{SpaceMetric, TimeMetric};

    pub fn test_movement(movement: impl Movement, expected_positions: impl Iterator<Item=(TimeMetric, Point3<SpaceMetric>)>) {
        expected_positions.for_each(|(time, position)| {
            assert_eq!(movement.get_position_at(time), position)
        });
    }
}