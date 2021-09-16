use std::cell::RefCell;
use std::rc::Rc;

use crate::body::Body;
use crate::event::Event;
use crate::ipn_sim::ipn_sim::IpnSim;
use crate::node::message_buffer::MessageBuffer;
use crate::movement::Movement;
use crate::node::node::Node;
use crate::report::Report;
use crate::router::Router;
use crate::schedule::schedule::Schedule;
use crate::transceiver::transceiver::Transceiver;
use crate::utils::{NodeId, shared, SpaceMetric, TimeMetric};
use crate::utils::Shared;
use crate::node::node_builder::NodeBuilder;

pub struct IpnSimBuilder {
    sim_length: TimeMetric,
    reports: Vec<Shared<dyn Report>>,
    schedule: Schedule<TimeMetric, Box<dyn Event>>,
    nodes: Vec<Shared<Node>>,
    bodies: Vec<Shared<Body>>,
}

impl IpnSimBuilder {
    pub fn new(sim_length: TimeMetric) -> Self {
        Self {
            sim_length,
            reports: vec![],
            schedule: Schedule::new(),
            nodes: vec![],
            bodies: vec![],
        }
    }

    pub fn add_report(mut self, report: impl Report + 'static) -> Self {
        self.reports.push(shared(report));
        self
    }

    pub fn add_shared_report(mut self, report: &Shared<impl Report + 'static>) -> Self {
        self.reports.push(report.clone());
        self
    }

    pub fn add_dyn_report(mut self, report: &Shared<dyn Report>) -> Self {
        self.reports.push(Rc::clone(report));
        self
    }

    pub fn add_event(mut self, time: TimeMetric, event: impl Event + 'static) -> Self {
        self.schedule.insert_event(time, Box::new(event));
        self
    }

    pub fn add_node(
        mut self,
        builder: &NodeBuilder
    ) -> Self {
        self.nodes.push(shared(builder.build(self.nodes.len() as NodeId)));
        self
    }

    pub fn add_body(
        mut self,
        name: impl Into<String>,
        mass: SpaceMetric,
        movement: impl Movement + 'static,
        radius: SpaceMetric,
    ) -> Self {
        self.bodies.push(shared(Body {
            name: name.into(),
            mass,
            position: movement.get_position_at(0),
            movement: Box::new(movement),
            radius,
        }));
        self
    }

    pub fn get_node(&self, node_index: usize) -> Shared<Node> {
        Rc::clone(&self.nodes[node_index])
    }

    pub fn get_body(&self, body_index: usize) -> Shared<Body> {
        Rc::clone(&self.bodies[body_index])
    }

    pub fn build(self) -> IpnSim {
        IpnSim {
            time: 0,
            length: self.sim_length,
            schedule: self.schedule,
            reports: if self.reports.is_empty() {
                None
            } else {
                Some(self.reports)
            },
            nodes: Some(self.nodes),
            bodies: self.bodies,
        }
    }
}
