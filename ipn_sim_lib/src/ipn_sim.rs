use crate::event::Event;
use crate::node::Node;
use crate::schedule::schedule::Schedule;
use crate::tick_result::TickResult;
use crate::utils::TimeMetric;

pub struct IpnSim {
    pub time: TimeMetric,
    pub length: TimeMetric,
    pub schedule: Schedule<TimeMetric, Box<dyn Event>>,
    pub nodes: Vec<Node>,
}

impl IpnSim {
    pub fn new(length: TimeMetric, nodes: Vec<Node>) -> Self {
        Self {
            time: 0,
            length,
            schedule: Schedule::new(),
            nodes,
        }
    }

    pub fn tick(&mut self) -> TickResult {
        if let Some((time, events)) = self.schedule.pop_next_events() {
            if time > self.length {
                return TickResult::SimulationEnd;
            }

            self.time = time;
            for node in &mut self.nodes {
                node.set_position(time);
            }

            for mut event in events {
                event.handle(self);
            }

            TickResult::MoreEvents
        } else {
            TickResult::NoMoreEvents
        }
    }

    pub fn run(&mut self) -> TickResult {
        while let result = self.tick() {
            if result.is_terminal() {
                return result;
            }
        }
        unreachable!()
    }

    pub fn add_event(&mut self, time: TimeMetric, event: impl Event + 'static) {
        self.schedule.insert_event(time, Box::new(event))
    }
}