use crate::ipn_sim::ipn_sim::IpnSim;
use crate::event::Event;

pub trait Report {
    fn on_init(&mut self, sim: &IpnSim) {}

    fn on_tick(&mut self, sim: &IpnSim, events: &Vec<Box<dyn Event>>) {}

    fn on_end(&mut self, sim: &IpnSim) {}
}