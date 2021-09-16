use std::collections::HashMap;

use crate::utils::Data;
use rustc_hash::FxHashMap;

pub type MessageHandle = usize;

pub struct MessageBuffer {
    pub buffer: FxHashMap<MessageHandle, Data>,
    next_handle: MessageHandle,
    pub size: usize,
    pub capacity: usize,
}

impl MessageBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: FxHashMap::default(),
            next_handle: 0,
            size: 0,
            capacity,
        }
    }

    pub fn add_message(&mut self, data: Data) -> Option<MessageHandle> {
        if self.size + data.len() > self.capacity {
            return None;
        }
        let handle = self.next_handle;
        self.next_handle += 1;
        self.size += data.len();
        self.buffer.insert(handle, data);
        Some(handle)
    }

    pub fn get_message(&self, handle: &MessageHandle) -> Option<&Data> {
        self.buffer.get(handle)
    }

    pub fn remove_message(&mut self, handle: &MessageHandle) -> Option<Data> {
        self.buffer.remove(handle).map(|message| {
            self.size -= message.len();
            message
        })
    }

    pub fn get_occupancy(&self) -> f32 {
        self.size as f32 / self.capacity as f32
    }
}
