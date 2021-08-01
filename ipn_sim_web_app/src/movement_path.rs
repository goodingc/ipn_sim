use ipn_sim_lib::cgmath::Point3;
use ipn_sim_lib::downcast_rs::*;
use ipn_sim_lib::movement::Movement;
use ipn_sim_lib::movements::orbital_movement::OrbitalMovement;
use ipn_sim_lib::movements::path_movement::PathMovement;
use ipn_sim_lib::movements::static_movement::StaticMovement;
use ipn_sim_lib::utils::{SpaceMetric, TimeMetric};

pub trait MovementPath: Movement {
    fn get_path(&self, interval: TimeMetric, length: TimeMetric) -> Vec<Point3<SpaceMetric>> {
        (0..length)
            .step_by(interval as usize)
            .map(|time| self.get_position_at(time))
            .collect::<Vec<Point3<SpaceMetric>>>()
    }

    fn try_get_path(
        movement: &Box<dyn Movement>,
        interval: TimeMetric,
        length: TimeMetric,
    ) -> Option<Vec<Point3<SpaceMetric>>>
    where
        Self: Sized,
    {
        movement
            .downcast_ref()
            .map(|movement: &Self| movement.get_path(interval, length))
    }
}

impl MovementPath for OrbitalMovement {
    fn get_path(&self, interval: TimeMetric, length: TimeMetric) -> Vec<Point3<SpaceMetric>> {
        (0..(self.period.abs().min(length as f64)) as TimeMetric)
            .step_by(interval as usize)
            .map(|time| self.get_position_at(time))
            .collect::<Vec<Point3<SpaceMetric>>>()
    }
}

impl MovementPath for StaticMovement {
    fn get_path(&self, interval: TimeMetric, length: TimeMetric) -> Vec<Point3<SpaceMetric>> {
        vec![self.point]
    }
}

impl MovementPath for PathMovement {}

pub fn get_movement_path(
    movement: &Box<dyn Movement>,
    interval: TimeMetric,
    length: TimeMetric,
) -> Vec<Point3<SpaceMetric>> {
    OrbitalMovement::try_get_path(movement, interval, length)
        .or_else(|| StaticMovement::try_get_path(movement, interval, length))
        .or_else(|| PathMovement::try_get_path(movement, interval, length))
        .expect("No movement path implementation!")
}
