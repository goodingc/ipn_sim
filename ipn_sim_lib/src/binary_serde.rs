use crate::utils::Data;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashSet;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub trait BinarySerde: Serialize + DeserializeOwned + Hash {
    fn as_data(&self) -> Data {
        // let mut hasher = DefaultHasher::new();
        // self.hash(&mut hasher);
        // let hash = hasher.finish();
        // if !SER_HASHES.lock().unwrap().insert(hash) {
        //     SER_HITS.fetch_add(1, Ordering::SeqCst);
        // }
        // SER_CALLS.fetch_add(1, Ordering::SeqCst);

        let data = bincode::serialize(self).unwrap();

        // let mut hasher = DefaultHasher::new();
        // data.hash(&mut hasher);
        // let hash = hasher.finish();
        // DE_HASHES.lock().unwrap().insert(hash);

        data
    }

    fn from_data(data: &Data) -> Self {
        // let mut hasher = DefaultHasher::new();
        // data.hash(&mut hasher);
        // let hash = hasher.finish();
        // if !DE_HASHES.lock().unwrap().insert(hash) {
        //     DE_HITS.fetch_add(1, Ordering::SeqCst);
        // }
        // DE_CALLS.fetch_add(1, Ordering::SeqCst);

        let value: Self = bincode::deserialize(data.as_slice()).unwrap();

        // let mut hasher = DefaultHasher::new();
        // value.hash(&mut hasher);
        // let hash = hasher.finish();
        // SER_HASHES.lock().unwrap().insert(hash);

        value
    }
}

lazy_static! {
    pub static ref SER_HASHES: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
    pub static ref DE_HASHES: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
}

pub static SER_CALLS: AtomicUsize = AtomicUsize::new(0);
pub static SER_HITS: AtomicUsize = AtomicUsize::new(0);

pub static DE_CALLS: AtomicUsize = AtomicUsize::new(0);
pub static DE_HITS: AtomicUsize = AtomicUsize::new(0);