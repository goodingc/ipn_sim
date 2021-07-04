use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::utils::TimeMetric;
use crate::sim_wrapper::interval_event::IntervalEvent;
use crate::sim_wrapper::setup_data::SetupData;
use ipn_sim_lib::tick_result::TickResult;
use crate::sim_wrapper::tick_data::TickData;

pub struct SimWrapper {
    pub(crate) sim: IpnSim,
    interval: TimeMetric,
}

impl SimWrapper {
    pub fn wrap(mut sim: IpnSim, interval: TimeMetric) -> Self {
        sim.add_event(0, IntervalEvent(interval));
        sim.init();
        Self {
            sim,
            interval,
        }
    }

    pub fn get_setup_data(&self) -> SetupData {
        SetupData {
            nodes: self.sim.nodes.as_ref().unwrap()
        }
    }

    pub fn tick(&mut self) -> TickData {
        loop {
            let result = self.sim.tick();

            if self.sim.time % self.interval == 0 {
                return TickData {
                    final_tick: result.is_terminal(),
                    time: self.sim.time,
                    nodes: self.sim.nodes.as_ref().unwrap(),
                };
            }
        }
    }
}