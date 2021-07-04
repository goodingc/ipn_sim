use std::cell::RefCell;
use std::rc::Rc;

use crate::ipn_sim::ipn_sim::IpnSim;
use crate::message_buffer::MessageBuffer;
use crate::movement::Movement;
use crate::node::Node;
use crate::report::Report;
use crate::router::Router;
use crate::schedule::schedule::Schedule;
use crate::utils::{NodeId, TimeMetric};
use crate::transceiver::transceive_guard::TransceiveGuard;
use crate::transceiver::transceiver::Transceiver;

pub struct IpnSimBuilder {
    sim_length: TimeMetric,
    reports: Vec<Box<dyn Report>>,
    nodes: Vec<Rc<RefCell<Node>>>,
}

impl IpnSimBuilder {
    pub fn new(sim_length: TimeMetric) -> Self {
        Self {
            sim_length,
            reports: vec![],
            nodes: vec![],
        }
    }

    pub fn add_report(mut self, report: impl Report + 'static) -> Self {
        self.reports.push(Box::new(report));
        self
    }

    pub fn add_node(mut self, name: impl Into<String>, movement: impl Movement + 'static, router: impl Router + 'static, transceiver: Transceiver) -> Self {
        self.nodes.push(Rc::new(RefCell::new(Node {
            id: self.nodes.len() as NodeId,
            name: name.into(),
            position: movement.get_position_at(0),
            movement: Box::new(movement),
            message_buffer: MessageBuffer::new(),
            router: Some(Box::new(router)),
            transceiver
        })));
        self
    }

    pub fn build(self) -> IpnSim {
        IpnSim {
            time: 0,
            length: self.sim_length,
            schedule: Schedule::new(),
            reports: if self.reports.is_empty() { None } else { Some(self.reports) },
            nodes: Some(self.nodes),
        }
    }
}