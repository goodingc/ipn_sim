use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use ipn_sim_lib::event::Event;
use ipn_sim_lib::events::router_event::{RouterEvent, RouterEventType};
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::node::node::Node;
use ipn_sim_lib::report::Report;
use ipn_sim_lib::utils::{MessageId, Shared};
use graph_layout::vertex::Vertex;
use graph_layout::graph_layout::GraphLayout;
use ipn_sim_lib::message_destination::MessageDestination;
use graph_layout::petgraph::Graph;

#[derive(Default)]
pub struct SingleMessageGraph {
    pub message_data: HashMap<MessageId, MessageData>,
    pub active_data: Option<MessageGraphData>,
}

pub struct MessageData {
    pub source_node: Shared<Node>,
    pub destination: MessageDestination<Shared<Node>>,
    pub connections: Vec<(Shared<Node>, Shared<Node>, bool)>,
}

#[derive(Default)]
pub struct MessageGraphData {
    pub graph_layout: GraphLayout,
    pub node_vertices: Vec<(Shared<Node>, Shared<Vertex>)>,
}

impl SingleMessageGraph {
    pub fn set_active_data(&mut self, message_id: MessageId) {
        let mut graph_layout = GraphLayout::new(Graph::new(), 20., 0.1, 0.3, 1.8);

        let data = self.message_data
            .get(&message_id)
            .unwrap();

        let mut node_id_map = HashMap::new();

        for (node_1, node_2, _) in &data.connections {
            for node in [node_1, node_2] {
                node_id_map
                    .entry(node.borrow().id)
                    .or_insert_with(|| Rc::clone(node));
            }
        }

        // let node_id_vertices = graph_layout
        //     .fill_from_edges(
        //         data.connections
        //             .iter()
        //             .map(|(node_1, node_2, _)| (node_1.borrow().id, node_2.borrow().id))
        //             .collect()
        //     );
        //
        // self.active_data = Some(MessageGraphData {
        //     graph_layout,
        //     node_vertices: node_id_vertices
        //         .into_iter()
        //         .map(|(node_id, vertex)| (node_id_map.remove(&node_id).unwrap(), vertex))
        //         .collect(),
        // });

        self.active_data = None;
    }
}

impl Report for SingleMessageGraph {
    fn on_tick(&mut self, _sim: &IpnSim, events: &Vec<Box<dyn Event>>) {
        for event in events {
            if let Some(router_event) = event.downcast_ref::<RouterEvent>() {
                match &router_event.event_type {
                    RouterEventType::MessageCreated {
                        id, destination, ..
                    } => {
                        self.message_data.insert(*id, MessageData {
                            source_node: Rc::clone(&router_event.node),
                            destination: destination.clone(),
                            connections: vec![]
                        });
                    }
                    RouterEventType::MessageReceived {
                        id,
                        source_node,
                    } => {
                        self.message_data
                            .get_mut(&id)
                            .unwrap()
                            .connections
                            .push((Rc::clone(source_node), Rc::clone(&router_event.node), false));
                    }
                    _ => {}
                }
            }
        }
    }
}