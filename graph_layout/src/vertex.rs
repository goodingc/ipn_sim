use cgmath::{Point2, Vector2, Zero};

pub struct Vertex {
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub frozen: bool,
}

impl Vertex {
    pub fn new(position: Point2<f32>) -> Self {
        Self {
            position,
            velocity: Vector2::zero(),
            acceleration: Vector2::zero(),
            frozen: false,
        }
    }
}