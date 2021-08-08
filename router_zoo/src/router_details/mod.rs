use ipn_sim_lib::router::Router;

pub mod vanilla_epidemic;
pub mod ack_epidemic;


pub trait RouterDetails {
    fn name() -> &'static str;

    fn params() -> Box<[(&'static str, RouterParamType)]>;

    fn default_params() -> Box<[String]>;

    fn build_instance(string_params: Box<[String]>) -> Self;
}

pub enum RouterParamType {
    Number
}

