use std::collections::HashMap;

use bit_vec::BitVec;
use serde::Serialize;
use typescript_definitions::TypescriptDefinition;
use wasm_bindgen::prelude::*;

use crate::utils::Data;

#[derive(Serialize, Clone, TypescriptDefinition)]
pub struct MessageBuffer {
    pub buffer: HashMap<usize, Data>,

    #[serde(skip)]
    next_handle: usize,

    size: usize,
}

impl MessageBuffer {
    pub fn new() -> Self {
        Self {
            buffer: HashMap::new(),
            next_handle: 0,
            size: 0,
        }
    }

    pub fn add_message(&mut self, data: Data) -> usize {
        let handle = self.next_handle;
        self.next_handle += 1;
        self.size += data.len();
        self.buffer.insert(handle, data);
        handle
    }

    pub fn get_message(&self, id: usize) -> Option<&Data> {
        self.buffer.get(&id)
    }

    pub fn remove_message(&mut self, id: usize) -> Option<Data> {
        self.buffer.remove(&id).map(|message| {
            self.size -= message.len();
            message
        })
    }
}