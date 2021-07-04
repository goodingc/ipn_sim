use ipn_sim_lib::utils::TimeMetric;
use ipn_sim_lib::event::Event;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;

#[test]
fn test() {
    let mut sim = IpnSimBuilder::new(10000).build();
    sim.add_event(0, IntervalEvent(100));
}

pub struct IntervalEvent(TimeMetric);

impl Event for IntervalEvent {
    fn handle(self: Box<Self>, sim: &mut IpnSim) {
        sim.schedule.insert_event(sim.time + self.0, Box::new(Self(self.0)))
    }
}