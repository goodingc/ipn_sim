use crate::utils::{Data, NodeId};
use crate::router_link::RouterLink;

pub trait Router {
    fn on_init(&mut self, link: &mut RouterLink, id: NodeId) {}

    fn on_message_created(&mut self, link: &mut RouterLink, data: Data) {}

    fn on_data_received(&mut self, link: &mut RouterLink, data: Data) {}
    //
    // fn on_awake(&mut self, link: &mut NodeLink) {}
}