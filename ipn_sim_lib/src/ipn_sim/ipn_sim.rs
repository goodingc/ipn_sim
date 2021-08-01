use std::cell::RefCell;
use std::rc::Rc;

use cgmath::{InnerSpace, MetricSpace};

use crate::body::Body;
use crate::event::Event;
use crate::ipn_sim::tick_result::TickResult;
use crate::node::Node;
use crate::report::Report;
use crate::router::Router;
use crate::router_link::RouterLink;
use crate::schedule::schedule::Schedule;
use crate::utils;
use crate::utils::{Data, NodeId, TimeMetric};
use bit_vec::BitVec;
use collision::{Continuous, Ray, Sphere};
use std::mem::forget;
use crate::utils::Shared;


pub struct IpnSim {
    pub time: TimeMetric,
    pub length: TimeMetric,
    pub schedule: Schedule<TimeMetric, Box<dyn Event>>,
    pub reports: Option<Vec<Shared<dyn Report>>>,
    pub nodes: Option<Vec<Shared<Node>>>,
    pub bodies: Vec<Shared<Body>>,
}

impl IpnSim {
    pub fn init(&mut self) {
        let nodes = self.nodes.take().unwrap();
        for node in &nodes {
            let mut node_ref = node.borrow_mut();
            let node_id = node_ref.id;
            let mut router = node_ref.router.take().unwrap();
            router.on_init(&mut RouterLink::new(&mut node_ref, node, self), node_id);
            node_ref.router = Some(router);
        }
        self.nodes = Some(nodes);

        if self.reports.is_some() {
            let mut reports = self.reports.take().unwrap();
            for report in &mut reports {
                report.borrow_mut().on_init(&self)
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

            for body in &self.bodies {
                body.borrow_mut().set_position(time);
            }

            let mut nodes = self.nodes.take().unwrap();
            for node in &mut nodes {
                node.borrow_mut().set_position(time);
            }
            self.nodes = Some(nodes);

            let report_events = if self.reports.is_some() {
                Some(events.iter().map(|event| (*event).clone()).collect())
            } else {
                None
            };

            for event in events {
                event.handle(self);
            }

            if let Some(events) = report_events {
                let mut reports = self.reports.take().unwrap();
                for report in &mut reports {
                    report.borrow_mut().on_tick(&self, &events);
                }
                self.reports = Some(reports)
            }

            TickResult::MoreEvents
        } else {
            TickResult::NoMoreEvents
        }
    }

    pub fn run(&mut self) -> TickResult {
        self.init();
        loop {
            let result = self.tick();
            if result.is_terminal() {
                return result;
            }
        }
    }

    pub fn end(&mut self) {
        self.reports.take().map(|mut reports| {
            for report in &reports {
                report.borrow_mut().on_end(&self)
            }
        });
    }

    pub fn add_event(&mut self, time: TimeMetric, event: impl Event + 'static) {
        self.schedule.insert_event(time, Box::new(event))
    }

    pub fn get_node(&self, node_index: NodeId) -> Shared<Node> {
        Rc::clone(&self.nodes.as_ref().unwrap()[node_index as usize])
    }

    pub fn nodes_can_transceive(&self, transmitting_node: &Node, receiving_node: &Node) -> bool {
        transmitting_node
            .transceiver
            .guard
            .can_transceive(transmitting_node, receiving_node)
            && receiving_node
                .transceiver
                .guard
                .can_transceive(receiving_node, transmitting_node)
            && self.nodes_are_covisible(transmitting_node, receiving_node)
    }

    pub fn nodes_are_covisible(&self, transmitting_node: &Node, receiving_node: &Node) -> bool {
        let source_position = transmitting_node.position;
        let destination_position = receiving_node.position;
        let ray = Ray::new(
            source_position,
            (destination_position - source_position).normalize(),
        );
        for body in &self.bodies {
            let body = body.borrow();
            let collider = Sphere {
                center: body.position,
                radius: body.radius,
            };
            if let Some(intersection) = collider.intersection(&ray) {
                if source_position.distance(intersection)
                    < source_position.distance(destination_position)
                {
                    return false;
                }
            }
        }
        true
    }
}
