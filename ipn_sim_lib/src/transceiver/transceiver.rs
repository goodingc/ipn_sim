use std::collections::VecDeque;

use crate::transceiver::transceive_guard::TransceiveGuard;
use crate::utils::{Data, TimeMetric};

pub struct Transceiver {
    pub transmit_speed: f64,

    pub busy_until: TimeMetric,

    buffer: VecDeque<Data>,

    pub guard: Box<dyn TransceiveGuard>,
}

impl Transceiver {
    pub fn new(transmit_speed: f64, guard: Box<dyn TransceiveGuard>) -> Self {
        Self {
            transmit_speed,
            busy_until: 0,
            buffer: VecDeque::new(),
            guard,
        }
    }

    pub fn add_to_buffer(&mut self, data: Data, current_time: TimeMetric) -> TimeMetric {
        let transmit_time = self.get_transmit_time(&data);
        self.buffer.push_back(data);
        let transmit_start = current_time.max(self.busy_until);
        self.busy_until = transmit_start + transmit_time;
        transmit_start
    }

    pub fn pop_head_data(&mut self) -> Data {
        self.buffer.pop_front().unwrap()
    }

    pub fn get_transmit_time(&self, data: &Data) -> TimeMetric {
        (data.len() as f64 / self.transmit_speed) as TimeMetric
    }
}
