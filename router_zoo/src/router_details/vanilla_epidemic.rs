use crate::router_details::{RouterDetails, RouterParamType};
use ipn_sim_lib::routers::epidemic::epidemic::Epidemic;
use ipn_sim_lib::routers::epidemic::flavours::vanilla::Vanilla;

impl RouterDetails for Epidemic<Vanilla> {

    fn name() -> &'static str {
        "Vanilla Epidemic"
    }

    fn params() -> Box<[(&'static str, RouterParamType)]> {
        Box::new([
            ("Vector Size", RouterParamType::Number),
            ("Reconnect Time", RouterParamType::Number),
        ])
    }

    fn default_params() -> Box<[String]> {
        Box::new([
            (1024 * 8).to_string(),
            (1_000_000_000 * 3600).to_string()
        ])
    }

    fn build_instance(string_params: Box<[String]>) -> Self {
        Self::new(string_params[0].parse().unwrap(), 1_000_000_000 * 3600)
    }
}