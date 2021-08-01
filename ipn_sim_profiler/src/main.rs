use crate::profiler::Profiler;
use crate::profiler_report::ProfilerReport;

pub mod factories;
pub mod profiler;
pub mod profiler_report;

fn main() {
    Profiler::new()
        .run_scenario("orbiting rings", factories::orbiting_rings, 5)
        .report();
}
