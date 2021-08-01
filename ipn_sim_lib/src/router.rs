use crate::events::create_message_event::MessageDestination;
use crate::router_link::RouterLink;
use crate::utils::{Data, NodeId, TimeMetric};

pub trait Router {
    fn on_init(&mut self, _link: &mut RouterLink, _id: NodeId) {}

    fn on_message_created(
        &mut self,
        _link: &mut RouterLink,
        destination: MessageDestination,
        payload: Data,
        ttl: Option<TimeMetric>,
    ) {
    }

    fn on_data_received(&mut self, _link: &mut RouterLink, _data: Data) {}

    fn on_awake(&mut self, link: &mut RouterLink) {}
}
