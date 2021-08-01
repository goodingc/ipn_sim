use crate::router::Router;
use crate::router_link::RouterLink;
use crate::utils::Data;

#[derive(Clone)]
pub struct TestRouter;

impl Router for TestRouter {
    // fn on_message_created(&mut self, link: &mut RouterLink, data: Data) {
    //     link.add_to_message_buffer(data.clone());
    //     link.add_to_transmit_buffer(data);
    //     link.log("message created");
    // }
    //
    // fn on_data_received(&mut self, link: &mut RouterLink, data: Data) {
    //     link.add_to_message_buffer(data);
    //     link.log("data received");
    // }
}
