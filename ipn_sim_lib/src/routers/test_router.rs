use crate::router::Router;
use crate::router_link::RouterLink;
use crate::utils::Data;

pub struct TestRouter;

impl Router for TestRouter {
    fn on_message_created(&mut self, link: &mut RouterLink, data: Data) {
        link.add_to_message_buffer(data);
    }
}