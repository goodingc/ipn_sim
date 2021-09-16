use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

use cgmath::{EuclideanSpace, Euler, Point3, Quaternion, Rad, Rotation, Vector3};

use crate::body::Body;
use crate::movement::Movement;
use crate::utils::{G, SpaceMetric, TimeMetric};
use crate::utils::Shared;

#[derive(Clone)]
pub struct OrbitalMovement {
    central_body: Shared<Body>,
    semimajor_axis: SpaceMetric,
    inclination: SpaceMetric,
    pub period: SpaceMetric,
    offset: SpaceMetric,
    ascending_node_offset: SpaceMetric,
}

impl OrbitalMovement {
    pub fn new(
        central_body: &Shared<Body>,
        semimajor_axis: SpaceMetric,
        inclination: SpaceMetric,
        offset: SpaceMetric,
        ascending_node_offset: SpaceMetric,
        clockwise: bool,
    ) -> Self {
        Self {
            central_body: Rc::clone(central_body),
            semimajor_axis,
            inclination,
            period: 2.
                * PI as SpaceMetric
                * (semimajor_axis.powi(3) / (G * central_body.borrow().mass)).sqrt()
                * 1_000_000_000.
                * if clockwise { -1. } else { 1. },
            offset,
            ascending_node_offset,
        }
    }
}

impl Movement for OrbitalMovement {
    fn get_position_at(&self, time: TimeMetric) -> Point3<SpaceMetric> {
        Point3::from_vec(
            self.central_body
                .borrow()
                .movement
                .get_position_at(time)
                .to_vec() +
                Quaternion::from(Euler::new(
                    Rad(0.),
                    Rad(self.ascending_node_offset),
                    Rad(0.),
                )) * Quaternion::from(Euler::new(
                    Rad(self.inclination),
                    Rad(time as SpaceMetric / self.period * 2. * (PI as SpaceMetric) + self.offset),
                    Rad(0.),
                )).rotate_vector(Vector3::new(self.semimajor_axis, 0., 0.)),
        )
    }
}
