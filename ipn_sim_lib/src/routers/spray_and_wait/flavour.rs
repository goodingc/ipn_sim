use crate::binary_serde::BinarySerde;

pub trait Flavour: Clone {
    type PingPacket: BinarySerde;
    type FulfillmentPacket: BinarySerde;

    fn new() -> Self;
}