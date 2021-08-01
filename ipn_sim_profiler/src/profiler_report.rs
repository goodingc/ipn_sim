use crate::profiler::ProfilerData;
use ipn_sim_lib::event::Event;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::report::Report;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use ipn_sim_lib::utils::Shared;

pub struct ProfilerReport {
    run_data: Shared<Vec<ProfilerData>>,
    init_time: Option<SystemTime>,
    elapsed_time: Option<Duration>,
    processed_events: u64,
}

impl ProfilerReport {
    pub fn new(run_data: &Shared<Vec<ProfilerData>>) -> Self {
        Self {
            run_data: Rc::clone(run_data),
            init_time: None,
            elapsed_time: None,
            processed_events: 0,
        }
    }
}

impl Report for ProfilerReport {
    fn on_init(&mut self, _sim: &IpnSim) {
        self.init_time = Some(SystemTime::now())
    }

    fn on_tick(&mut self, _sim: &IpnSim, events: &Vec<Box<dyn Event>>) {
        self.processed_events += events.len() as u64;
    }

    fn on_end(&mut self, sim: &IpnSim) {
        self.run_data.borrow_mut().push(ProfilerData {
            elapsed_time: self.init_time.unwrap().elapsed().unwrap(),
            sim_length: sim.length,
            processed_events: self.processed_events,
        })
    }
}
