use crate::router_link::RouterLink;
use crate::utils::{Data, NodeId, TimeMetric};
use dyn_clonable::clonable;
use crate::message_destination::MessageDestination;

#[clonable]
pub trait Router: Clone {
    fn on_init(&mut self, _link: &mut RouterLink, _id: NodeId) {}

    fn on_message_created(
        &mut self,
        _link: &mut RouterLink,
        destination: MessageDestination<NodeId>,
        payload: Data,
        ttl: Option<TimeMetric>,
    ) {
    }

    fn on_data_received(&mut self, _link: &mut RouterLink, _data: Data) {}

    fn on_awake(&mut self, link: &mut RouterLink) {}
}
