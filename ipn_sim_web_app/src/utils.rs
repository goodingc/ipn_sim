use ipn_sim_lib::cgmath::Point3;
use ipn_sim_lib::utils::SpaceMetric;

pub fn format_position(position: Point3<SpaceMetric>) -> String {
    format!(
        "({:.2}km, {:.2}km, {:.2}km)",
        position.x / 1000.,
        position.y / 1000.,
        position.z / 1000.,
    )
}