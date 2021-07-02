use crate::ipn_sim::IpnSim;

pub trait Event {
    fn handle(&mut self, sim: &mut IpnSim);
}