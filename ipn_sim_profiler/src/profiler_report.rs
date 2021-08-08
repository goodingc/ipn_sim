use ipn_sim_lib::event::Event;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::report::Report;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use ipn_sim_lib::utils::{Shared, TimeMetric};

pub struct ProfilerReport {
    init_time: Option<SystemTime>,
    pub sim_length: Option<TimeMetric>,
    pub elapsed_time: Option<Duration>,
    pub processed_events: u64,
}

impl ProfilerReport {
    pub fn new() -> Self {
        Self {
            init_time: None,
            sim_length: None,
            elapsed_time: None,
            processed_events: 0,
        }
    }
}

impl Report for ProfilerReport {
    fn on_init(&mut self, sim: &IpnSim) {
        self.init_time = Some(SystemTime::now());
        self.sim_length = Some(sim.length);
    }

    fn on_tick(&mut self, _sim: &IpnSim, events: &Vec<Box<dyn Event>>) {
        self.processed_events += events.len() as u64;
    }

    fn on_end(&mut self, _sim: &IpnSim) {
        self.elapsed_time = Some(self.init_time.unwrap().elapsed().unwrap())
    }
}
