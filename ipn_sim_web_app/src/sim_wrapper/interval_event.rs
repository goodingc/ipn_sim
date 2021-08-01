use ipn_sim_lib::event::Event;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::utils;
use ipn_sim_lib::utils::TimeMetric;

#[derive(Clone)]
pub struct IntervalEvent(pub TimeMetric);

impl Event for IntervalEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        sim.schedule
            .insert_event(sim.time + self.0, Box::new(Self(self.0)));
    }
}
