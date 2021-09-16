use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use ipn_sim_lib::ipn_sim::ipn_sim_builder::IpnSimBuilder;
use ipn_sim_lib::utils::{Shared, shared, TimeMetric};

use crate::profiler_report::ProfilerReport;
use ipn_sim_reports::reports::message_states::MessageStates;

pub struct Profiler {
    data: HashMap<String, Vec<(Shared<ProfilerReport>, Shared<MessageStates>)>>,
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
        let name= name.into();
        self.data.insert(
            name.clone(),
            (0..runs)
                .map(|index| {
                    let profiler_report = shared(ProfilerReport::new());
                    let message_states_report = shared(MessageStates::new());
                    factory()
                        .add_shared_report(&profiler_report)
                        .add_shared_report(&message_states_report)
                        .build()
                        .run();
                    println!("Scenario: {}, run {} complete", name, index);
                    (profiler_report, message_states_report)
                }).collect()
        );
        self
    }

    pub fn report(&self) {
        for (name, reports) in &self.data {
            println!("Scenario: {}, runs: {}", name, reports.len());

            let average_elapsed_time = reports
                .iter()
                .map(|(report, _)| report.borrow().elapsed_time.unwrap())
                .sum::<Duration>() / reports.len() as u32;
            println!(
                "\tAverage elapsed time: {}ms",
                average_elapsed_time.as_millis()
            );

            let average_sim_rate = reports
                .iter()
                .map(|(report, _)|
                    report.borrow().sim_length.unwrap() as f32 / report.borrow().elapsed_time.unwrap().as_nanos() as f32
                ).sum::<f32>() / reports.len() as f32;
            println!("\tAverage simulation rate: {}", average_sim_rate);

            let event_processing_rate = reports
                .iter()
                .map(|(report, _)|
                    report.borrow().processed_events as f32 / report.borrow().elapsed_time.unwrap().as_millis() as f32
                ).sum::<f32>() / reports.len() as f32;

            println!(
                "\tAverage event processing rate: {} events/ms",
                event_processing_rate
            );

            let delivery_rate = reports
                .iter()
                .map(|(_, report)|
                    (report.borrow().delivered_message_counts.value as f32) / (report.borrow().created_message_counts.value as f32)
                ).sum::<f32>() / reports.len() as f32;

            println!(
                "\tAverage delivery rate: {}",
                delivery_rate
            );
        }
        // let calls = SER_CALLS.load(Ordering::SeqCst);
        // let hits = SER_HITS.load(Ordering::SeqCst);
        // let cache_entries = SER_HASHES.lock().unwrap().len();
        // let hit_rate = hits as f32 / calls as f32;
        //
        // println!("Serialize:\n\tCalls: {}\n\tHits: {}\n\tHit rate :{}\n\tCache entries: {}", calls, hits, hit_rate, cache_entries);
        //
        // let calls = DE_CALLS.load(Ordering::SeqCst);
        // let hits = DE_HITS.load(Ordering::SeqCst);
        // let cache_entries = DE_HASHES.lock().unwrap().len();
        // let hit_rate = hits as f32 / calls as f32;
        //
        // println!("De:\n\tCalls: {}\n\tHits: {}\n\tHit rate :{}\n\tCache entries: {}", calls, hits, hit_rate, cache_entries);
    }
}