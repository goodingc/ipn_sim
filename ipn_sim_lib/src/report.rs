use crate::event::Event;
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::utils::Shared;

pub trait Report {
    fn on_init(&mut self, _sim: &IpnSim) {}

    fn on_tick(&mut self, _sim: &IpnSim, _events: &Vec<Box<dyn Event>>) {}

    fn on_end(&mut self, _sim: &IpnSim) {}
}