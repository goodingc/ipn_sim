use std::slice::Iter;

pub struct Log<T: PartialEq, E> {
    event_groups: Vec<(T, Vec<E>)>,
}

impl<T: PartialEq, E> Log<T, E> {
    pub fn new() -> Self {
        Self {
            event_groups: vec![],
        }
    }

    pub fn add_events(&mut self, time: T, mut events: Vec<E>) {
        if let Some((last_time, last_events)) = self.event_groups.last_mut() {
            if *last_time == time {
                last_events.append(&mut events);
                return;
            }
        }
        self.event_groups.push((time, events))
    }

    pub fn get_group(&self, index: usize) -> Option<&(T, Vec<E>)> {
        self.event_groups.get(index)
    }

    pub fn len(&self) -> usize {
        self.event_groups.len()
    }

    pub fn iter(&self) -> Iter<(T, Vec<E>)> {
        self.event_groups.iter()
    }
}
