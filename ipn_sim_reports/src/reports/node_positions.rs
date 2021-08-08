use ipn_sim_lib::report::Report;
use ipn_sim_lib::ipn_sim::ipn_sim::IpnSim;
use ipn_sim_lib::event::Event;
use ipn_sim_lib::utils::{Shared, TimeMetric, SpaceMetric};
use ipn_sim_lib::node::node::Node;
use crate::graph_report::GraphReport;
use yew::prelude::*;

pub struct NodePositions {
    nodes: Option<Vec<Shared<Node>>>,
}

impl NodePositions {
    pub fn new() -> Self {
        Self {
            nodes: None
        }
    }
}

impl Report for NodePositions {
    fn on_init(&mut self, sim: &IpnSim) {
        self.nodes = Some(
            sim.nodes
                .as_ref()
                .unwrap()
                .iter()
                .cloned()
                .collect()
        )
    }
}

impl GraphReport for NodePositions {
    fn render_graph(&self, width: u16, height: u16, sim_time: TimeMetric) -> Html {
        let nodes_content = self.nodes.as_ref().map_or("".into(), |nodes| {
            let (width, height) = (width as SpaceMetric, height as SpaceMetric);
            let (x_min, x_max, z_min, z_max) = nodes
                .iter()
                .fold((0f64, 0f64, 0f64, 0f64), |(x_min, x_max, z_min, z_max), node| {
                    let node_position = node.borrow().position;
                    (x_min.min(node_position.x), x_max.max(node_position.x), z_min.min(node_position.z), z_max.max(node_position.z))
                });

            let (x_range, z_range) = (x_max - x_min, z_max - z_min);
            nodes
                .iter()
                .map(|node| {
                    let node_position = node.borrow().position;
                    let x_pos = (node_position.x - x_min) / x_range * width;
                    let y_pos = (node_position.z - z_min) / z_range * height;
                    html! {
                        <circle cx=x_pos.to_string() cy=y_pos.to_string() r="5"/>
                    }
                }).collect::<Html>()
        });
        html! {
            <svg style=format!("width: {}px; height: {}px", width, height)>
                {nodes_content}
            </svg>
        }
    }
}