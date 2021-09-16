use ipn_sim_lib::utils::{MessageId, TimeMetric, Shared};
use ipn_sim_lib::events::router_event::{RouterEvent, RouterEventType};
use ipn_sim_lib::report::Report;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::event::Event;
use std::collections::HashMap;
use ipn_sim_lib::node::node::Node;
use std::rc::Rc;
use ipn_sim_lib::message_destination::MessageDestination;

#[derive(Default)]
pub struct Messages {
    pub messages: HashMap<MessageId, Message>
}

pub struct Message {
    pub source: Shared<Node>,
    pub destination: MessageDestination<Shared<Node>>,
    pub time_created: TimeMetric,
    pub ttl: Option<TimeMetric>,
    pub state: State,
    pub copies: usize,
}

#[derive(Eq, PartialEq)]
pub enum State {
    InFlight,
    Delivered,
    Dropped
}

impl Report for Messages {
    fn on_tick(&mut self, sim: &IpnSim, events: &Vec<Box<dyn Event>>) {
        for event in events {
            if let Some(router_event) = event.downcast_ref::<RouterEvent>() {
                match &router_event.event_type {
                    RouterEventType::MessageCreated {
                        id,
                        destination,
                        ttl
                    } => {
                        self.messages.insert(*id, Message {
                            source: Rc::clone(&router_event.node),
                            destination: destination.clone(),
                            time_created: sim.time,
                            ttl: *ttl,
                            state: State::InFlight,
                            copies: 1,
                        });
                    },
                    RouterEventType::MessageReceived {
                        id,
                        source_node,
                    } => {
                        let message_mut = self.messages.get_mut(&id).unwrap();
                        message_mut.copies += 1;
                    }
                    RouterEventType::MessageDropped {
                        id, ..
                    } => {
                        let message = self.messages.get_mut(&id).unwrap();
                        message.copies -= 1;

                        if message.copies == 0 && message.state != State::Delivered {
                            message.state = State::Dropped;
                        }
                    }
                    RouterEventType::MessageDelivered {
                        id, source_node,
                    } => {
                        self.messages.get_mut(&id).unwrap().state = State::Delivered
                    }
                    _ => {}
                }
            }
        }
    }
}