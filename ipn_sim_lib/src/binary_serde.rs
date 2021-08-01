use crate::utils::Data;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub trait BinarySerde: Serialize + DeserializeOwned {
    fn as_data(&self) -> Data {
        bincode::serialize(self).unwrap()
    }

    fn from_data(data: &Data) -> Self {
        bincode::deserialize(data.as_slice()).unwrap()
    }
}
