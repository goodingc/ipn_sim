use serde::{Deserialize, Serialize};

use crate::binary_serde::BinarySerde;

#[derive(Serialize, Deserialize)]
pub enum Packet<P: BinarySerde, F: BinarySerde> {
    #[serde(bound(deserialize = "P: BinarySerde"))]
    Ping(P),
    #[serde(bound(deserialize = "F: BinarySerde"))]
    Fulfillment(F),
}

impl<P: BinarySerde, F: BinarySerde> BinarySerde for Packet<P, F> {}