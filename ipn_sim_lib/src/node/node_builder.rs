use crate::movement::Movement;
use crate::transceiver::transceiver::Transceiver;
use crate::router::Router;
use crate::transceiver::transceive_guard::TransceiveGuard;
use crate::utils::NodeId;
use crate::node::node::Node;
use crate::message_buffer::MessageBuffer;

pub struct NodeBuilder {
    name: Option<String>,
    movement: Option<Box<dyn Movement>>,
    message_buffer_size: Option<usize>,
    router: Option<Box<dyn Router>>,
    transceive_speed: Option<f64>,
    transceive_guard: Option<Box<dyn TransceiveGuard>>
}

impl NodeBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            movement: None,
            message_buffer_size: None,
            router: None,
            transceive_speed: None,
            transceive_guard: None
        }
    }

    pub fn name(&mut self, name: impl ToString) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn boxed_movement(&mut self, boxed_movement: Box<dyn Movement>) -> &mut Self {
        self.movement = Some(boxed_movement);
        self
    }

    pub fn movement(&mut self, movement: impl Movement + 'static) -> &mut Self {
        self.boxed_movement(Box::new(movement))
    }

    pub fn message_buffer_size(&mut self, message_buffer_size: usize) -> &mut Self {
        self.message_buffer_size = Some(message_buffer_size);
        self
    }

    pub fn boxed_router(&mut self, boxed_router: Box<dyn Router>) -> &mut Self {
        self.router = Some(boxed_router);
        self
    }

    pub fn router(&mut self, router: impl Router + 'static) -> &mut Self {
        self.boxed_router(Box::new(router))
    }

    pub fn transceive_speed(&mut self, transceive_speed: f64) -> &mut Self {
        self.transceive_speed = Some(transceive_speed);
        self
    }

    pub fn transceive_guard(&mut self, transceive_guard: impl TransceiveGuard + 'static) -> &mut Self {
        self.transceive_guard = Some(Box::new(transceive_guard));
        self
    }

    pub fn build(&self, node_id: NodeId) -> Node {
        let movement = self.movement.as_ref().unwrap().clone();
        Node {
            id: node_id,
            name: self.name.as_ref().unwrap().clone(),
            position: movement.get_position_at(0),
            movement,
            message_buffer: MessageBuffer::new(self.message_buffer_size.unwrap()),
            router: Some(self.router.as_ref().unwrap().clone()),
            transceiver: Transceiver::new(
                self.transceive_speed.unwrap(),
                self.transceive_guard.as_ref().unwrap().clone()
            ),
        }
    }
}