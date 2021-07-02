use ipn_sim_lib::utils::TimeMetric;
use ipn_sim_lib::event::Event;
use ipn_sim_lib::ipn_sim::IpnSim;

#[test]
fn test() {
    let mut sim = IpnSim::new(10000, vec![]);
    sim.add_event(0, IntervalEvent(100));
}

pub struct IntervalEvent(TimeMetric);

impl Event for IntervalEvent {
    fn handle(&mut self, sim: &mut IpnSim) {
        sim.schedule.insert_event(sim.time + self.0, Box::new(Self(self.0)))
    }
}