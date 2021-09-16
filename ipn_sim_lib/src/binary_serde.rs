use crate::utils::Data;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashSet;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::any::Any;
use rustc_hash::FxHasher;

pub trait BinarySerde: Serialize + DeserializeOwned {

    fn as_data(&self) -> Data {
        // let mut hasher = FxHasher::default();
        // self.hash(&mut hasher);
        // let hash = hasher.finish();
        //
        // SER_CACHE
        //     .lock()
        //     .unwrap()
        //     .entry(hash)
        //     .or_insert_with(|| bincode::serialize(self).unwrap().into_boxed_slice())
        //     .clone()
        bincode::serialize(self).unwrap().into_boxed_slice()
    }

    fn from_data(data: &Data) -> Self {
        bincode::deserialize(&*data).unwrap()
        // let mut hasher = DefaultHasher::new();
        // data.hash(&mut hasher);
        // let hash = hasher.finish();
        // //
        // *DE_CACHE
        //     .lock()
        //     .unwrap()
        //     .entry(hash)
        //     .or_insert_with(|| Box::new(bincode::deserialize(&*data).unwrap()))
        //     .clone()
    }
}

// lazy_static! {
//     static ref SER_CACHE: Mutex<HashMap<u64, Data>> = Mutex::new(HashMap::new());
//     static ref DE_CACHE: Mutex<HashMap<u64, Box<dyn Any + Send>>> = Mutex::new(HashMap::new());
// }

// lazy_static! {
//     pub static ref SER_HASHES: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
//     pub static ref DE_HASHES: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
// }
//
// pub static SER_CALLS: AtomicUsize = AtomicUsize::new(0);
// pub static SER_HITS: AtomicUsize = AtomicUsize::new(0);
//
// pub static DE_CALLS: AtomicUsize = AtomicUsize::new(0);
// pub static DE_HITS: AtomicUsize = AtomicUsize::new(0);