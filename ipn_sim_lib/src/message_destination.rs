use crate::utils::{NodeId, Shared};
use crate::node::node::Node;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum MessageDestination<T> {
    All,
    Single(T),
    Multiple(Vec<T>),
}

pub trait IsIncluded<T> {
    fn is_included(&self, other: T) -> bool;
}

impl IsIncluded<&NodeId> for MessageDestination<NodeId> {
    fn is_included(&self, other: &NodeId) -> bool {
        match self {
            MessageDestination::All => true,
            MessageDestination::Single(id) => *id == *other,
            MessageDestination::Multiple(ids) => ids.contains(other),
        }
    }
}

impl IsIncluded<&Shared<Node>> for MessageDestination<Shared<Node>> {
    fn is_included(&self, other: &Shared<Node>) -> bool {
        match self {
            MessageDestination::All => true,
            MessageDestination::Single(node) => Rc::ptr_eq(node, other),
            MessageDestination::Multiple(nodes) => nodes
                .iter()
                .any(|node| Rc::ptr_eq(node, other)),
        }
    }
}