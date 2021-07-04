use crate::ipn_sim::ipn_sim::IpnSim;

pub trait Event {
    fn handle(self: Box<Self>, sim: &mut IpnSim);
}