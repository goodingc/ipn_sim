pub mod format_time;

use std::cell::RefCell;
use std::rc::Rc;

use ipn_sim_lib::cgmath::Point3;
use ipn_sim_lib::utils::{SpaceMetric, Shared};
use yew::prelude::*;

use crate::sim_wrapper::sim_wrapper::SimWrapper;

pub fn format_position(position: Point3<SpaceMetric>) -> String {
    format!(
        "({:.2}km, {:.2}km, {:.2}km)",
        position.x / 1000.,
        position.y / 1000.,
        position.z / 1000.,
    )
}

#[derive(Properties, Clone)]
pub struct WrapperProps {
    pub wrapper: Shared<SimWrapper>,
}