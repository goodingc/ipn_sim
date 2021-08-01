use crate::profiler_report::ProfilerReport;
use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;
use ipn_sim_lib::utils::{TimeMetric, Shared, shared};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

pub struct Profiler {
    data: HashMap<String, Shared<Vec<ProfilerData>>>,
}

pub struct ProfilerData {
    pub elapsed_time: Duration,
    pub sim_length: TimeMetric,
    pub processed_events: u64,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn run_scenario(
        &mut self,
        name: impl Into<String>,
        factory: impl Fn() -> IpnSimBuilder,
        runs: usize,
    ) -> &mut Self {
        let run_data = shared(vec![]);
        self.data.insert(name.into(), Rc::clone(&run_data));
        for i in 0..runs {
            factory()
                .add_report(ProfilerReport::new(&run_data))
                .build()
                .run();
        }
        self
    }

    pub fn report(&self) {
        for (name, data) in &self.data {
            let data = data.borrow();
            println!("Scenario: {}, runs: {}", name, data.len());

            let average_elapsed_time =
                data.iter().map(|data| data.elapsed_time).sum::<Duration>() / data.len() as u32;
            println!(
                "  Average elapsed time: {:?}ms",
                average_elapsed_time.as_millis()
            );

            let average_sim_rate = data
                .iter()
                .map(|data| data.sim_length as f32 / data.elapsed_time.as_nanos() as f32)
                .sum::<f32>()
                / data.len() as f32;
            println!("  Average simulation rate: {:?}", average_sim_rate);

            let event_processing_rate = data
                .iter()
                .map(|data| data.processed_events as f32 / data.elapsed_time.as_millis() as f32)
                .sum::<f32>()
                / data.len() as f32;

            println!(
                "  Average event processing rate: {:?} events/ms",
                event_processing_rate
            );
        }
    }
}
