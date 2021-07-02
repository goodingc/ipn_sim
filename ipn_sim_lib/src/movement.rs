use cgmath::Vector3;
use crate::utils::{SpaceMetric, TimeMetric};

pub trait Movement {
    fn get_position_at(&self, time: TimeMetric) -> Vector3<SpaceMetric>;
}

#[cfg(test)]
pub mod tests {
    use crate::movement::Movement;
    use crate::utils::{TimeMetric, SpaceMetric};
    use cgmath::Vector3;

    pub fn test_movement(movement: impl Movement, expected_positions: impl Iterator<Item = (TimeMetric, Vector3<SpaceMetric>)>) {
        expected_positions.for_each(|(time, position)| {
            assert_eq!(movement.get_position_at(time), position)
        });
    }
}