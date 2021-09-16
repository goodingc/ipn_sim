use crate::{Shared, shared};
use crate::vertex::Vertex;
use cgmath::{Point2, EuclideanSpace, Vector2, Zero, MetricSpace, InnerSpace, Rad, Basis2, Rotation2, Rotation};
use std::rc::Rc;
use yew::services::ConsoleService;
use std::hash::Hash;
use std::collections::{HashSet, HashMap};
use std::f32::consts::PI;
use petgraph::Graph;
use petgraph::graph::{NodeIndex, DefaultIx};

#[derive(Default)]
pub struct GraphLayout {
    pub graph: Graph<Vertex, ()>,
    pub target_separation: f32,
    pub centering_strength: f32,
    pub step_scale: f32,
    pub dampening_factor: f32,
}

impl GraphLayout {
    pub fn new(graph: Graph<Vertex, ()>, target_separation: f32, centering_strength: f32, step_scale: f32, dampening_factor: f32) -> Self {
        Self {
            graph,
            target_separation,
            centering_strength,
            step_scale,
            dampening_factor,
        }
    }

    pub fn new_connected_vertex(&mut self, connected_node_indices: Vec<NodeIndex>) -> Vertex {
        let position = match self.graph.node_count() {
            0 => Vector2::zero(),
            1 => Vector2::new(self.target_separation, 0.),
            _ => {
                let mean_connection_position = connected_node_indices
                    .iter()
                    .map(|vertex| self.graph[*vertex].position.to_vec())
                    .sum::<Vector2<f32>>() / connected_node_indices.len() as f32;

                mean_connection_position * (1. + self.target_separation / mean_connection_position.magnitude())
            }
        };
        Vertex::new(Point2::from_vec(position))
    }

    pub fn step(&mut self) -> bool {
        let centering_vector = self.graph
            .node_weights()
            .map(|vertex| vertex.position.to_vec())
            .sum::<Vector2<f32>>() / self.graph.node_count() as f32 * self.centering_strength;

        let node_indices = self.graph.node_indices().collect::<Vec<_>>();

        for index in &node_indices {
            let vertex = &mut self.graph[*index];
            if vertex.frozen {
                continue
            }

            let vertex_position = vertex.position;
            let mut acceleration = -centering_vector;

            for other_index in &node_indices {
                if index == other_index {
                    continue;
                }

                let other_position = self.graph[*other_index].position;

                acceleration += -self.target_separation.powi(2) /
                    vertex_position.distance2(other_position) *
                    (other_position - vertex_position)
            }

            self.graph[*index].acceleration = acceleration;
        }

        for edge_index in self.graph.edge_indices() {
            let (v_1_i, v_2_i) = self.graph.edge_endpoints(edge_index).unwrap();

            let v_1_pos = self.graph[v_1_i].position;
            let v_2_pos = self.graph[v_2_i].position;

            let force = v_1_pos.distance(v_2_pos) /
                self.target_separation * (v_2_pos - v_1_pos);

            let vertex_1 = &mut self.graph[v_1_i];

            if !vertex_1.frozen {
                vertex_1.acceleration += force;
            }

            let vertex_2 = &mut self.graph[v_2_i];

            if !vertex_2.frozen {
                vertex_2.acceleration -= force;
            }
        }

        let mut total_velocity = 0.;

        for vertex in self.graph.node_weights_mut() {
            if vertex.frozen {
                continue
            }

            let acceleration = vertex.acceleration * self.step_scale;
            vertex.velocity += acceleration;

            vertex.velocity /= self.dampening_factor;
            let velocity = vertex.velocity * self.step_scale;
            vertex.position += velocity;

            total_velocity += velocity.magnitude();
        }

        // ConsoleService::log(&total_velocity.to_string());
        // ConsoleService::log(&self.vertices.len().to_string());
        // ConsoleService::log(&(total_velocity / self.vertices.len() as f32).to_string());
        // ConsoleService::log(&(self.vertices.is_empty() ||(total_velocity / self.vertices.len() as f32) < 1.).to_string());

        total_velocity.is_nan() ||(total_velocity / self.graph.node_count() as f32) < 1.
    }

    pub fn settle(&mut self) {
        // let centering_strength = self.centering_strength;
        // self.centering_strength = 1.;
        //
        // let step_scale = self.step_scale;
        // self.step_scale = 0.6;
        //
        // let dampening_factor = self.dampening_factor;
        // self.dampening_factor = 4.;

        while !self.step() {
            ConsoleService::log("loop");
        }

        ConsoleService::log("settled");

        // self.centering_strength = centering_strength;
        // self.step_scale = step_scale;
        // self.dampening_factor = dampening_factor;
    }
}

