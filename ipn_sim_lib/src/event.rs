use crate::ipn_sim::ipn_sim::IpnSim;
use downcast_rs::{impl_downcast, Downcast};
use dyn_clonable::*;
use std::any::Any;

#[clonable]
pub trait Event: Clone + Downcast {
    fn handle(self: Box<Self>, sim: &mut IpnSim);

    fn is_internal() -> bool where Self: Sized {
        false
    }
}

impl_downcast!(Event);
