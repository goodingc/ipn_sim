use crate::router_details::{RouterDetails, RouterParamType};
use ipn_sim_lib::routers::epidemic::epidemic::Epidemic;
use ipn_sim_lib::routers::epidemic::flavours::ack::Ack;

#[derive(Clone)]
pub struct AckEpidemic;

impl RouterDetails for Epidemic<Ack> {

    fn name() -> &'static str {
        "Acknowledged Epidemic"
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
            (1_000_000_000u64 * 3600u64).to_string()
        ])
    }

    fn build_instance(string_params: Box<[String]>) -> Self {
        Self::new(string_params[0].parse().unwrap(), 1_000_000_000 * 3600)
    }
}