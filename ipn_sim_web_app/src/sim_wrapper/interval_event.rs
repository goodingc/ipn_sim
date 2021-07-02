use ipn_sim_lib::utils::TimeMetric;
use ipn_sim_lib::event::Event;
use ipn_sim_lib::ipn_sim::IpnSim;

pub struct IntervalEvent(TimeMetric);

impl IntervalEvent {

}

impl Event for IntervalEvent {
    fn handle(&mut self, sim: &mut IpnSim) {
        sim.schedule.insert_event(sim.time + self.0, Box::new(Self(self.0)))
    }
}