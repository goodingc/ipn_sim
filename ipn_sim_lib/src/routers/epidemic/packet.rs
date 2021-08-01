use serde::{Deserialize, Serialize};

use crate::binary_serde::BinarySerde;

#[derive(Serialize, Deserialize)]
pub enum Packet<P: BinarySerde, R: BinarySerde, F: BinarySerde> {
    #[serde(bound(deserialize = "P: BinarySerde"))]
    Ping(P),
    #[serde(bound(deserialize = "R: BinarySerde"))]
    Request(R),
    #[serde(bound(deserialize = "F: BinarySerde"))]
    Fulfillment(F),
}

impl<P: BinarySerde, R: BinarySerde, F: BinarySerde> BinarySerde for Packet<P, R, F> {}
