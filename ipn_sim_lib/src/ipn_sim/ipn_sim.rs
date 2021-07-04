use std::cell::RefCell;
use std::rc::Rc;

use crate::event::Event;
use crate::node::Node;
use crate::report::Report;
use crate::router::Router;
use crate::router_link::RouterLink;
use crate::schedule::schedule::Schedule;
use crate::tick_result::TickResult;
use crate::utils::TimeMetric;

pub struct IpnSim {
    pub time: TimeMetric,
    pub length: TimeMetric,
    pub schedule: Schedule<TimeMetric, Box<dyn Event>>,
    pub reports: Option<Vec<Box<dyn Report>>>,
    pub nodes: Option<Vec<Rc<RefCell<Node>>>>,
}

impl IpnSim {
    pub fn init(&mut self) {
        let mut nodes = self.nodes.take().unwrap();
        for node in &nodes {
            let node_id = node.borrow().id;
            node.borrow_mut().router.as_mut().unwrap().on_init(
                &mut RouterLink::new(node, self),
                node_id,
            )
        }
        self.nodes = Some(nodes);

        if self.reports.is_some() {
            let mut reports = self.reports.take().unwrap();
            for report in &mut reports {
                report.on_init(&self)
            }
            self.reports = Some(reports)
        }
    }

    pub fn tick(&mut self) -> TickResult {
        if let Some((time, events)) = self.schedule.pop_next_events() {
            if time > self.length {
                return TickResult::SimulationEnd;
            }
            self.time = time;

            let mut nodes = self.nodes.take().unwrap();
            for node in &mut nodes {
                node.borrow_mut().set_position(time);
            }
            self.nodes = Some(nodes);

            for mut event in events {
                event.handle(self);
            }

            TickResult::MoreEvents
        } else {
            TickResult::NoMoreEvents
        }
    }

    pub fn run(&mut self) -> TickResult {
        loop {
            let result = self.tick();
            if result.is_terminal() {
                return result;
            }
        }
    }

    pub fn add_event(&mut self, time: TimeMetric, event: impl Event + 'static) {
        self.schedule.insert_event(time, Box::new(event))
    }

    pub fn get_node(&self, node_index: usize) -> Rc<RefCell<Node>> {
        Rc::clone(&self.nodes.as_ref().unwrap()[node_index])
    }
}
