use crate::schedule::time_slot::TimeSlot;
use std::cmp::Ordering;

use std::collections::BTreeMap;
use std::mem;

pub struct Schedule<T: Ord, E> {
    map: BTreeMap<T, Vec<E>>,
}

impl<T: Ord, E> Schedule<T, E> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert_event(&mut self, time: T, event: E) {
        if let Some(events) = self.map.get_mut(&time) {
            events.push(event);
        } else {
            self.map.insert(time, vec![event]);
        }
    }
}

impl<T: Ord + Copy, E> Schedule<T, E> {
    pub fn peek_next_time(&self) -> Option<T> {
        self.map.iter().next().map(|(next_time, _)| *next_time)
    }

    pub fn pop_next_events(&mut self) -> Option<(T, Vec<E>)> {
        self.peek_next_time()
            .and_then(|next_time| self.map.remove_entry(&next_time))
    }
}

#[cfg(test)]
mod tests {
    use crate::schedule::schedule::Schedule;

    #[test]
    fn test() {
        let mut schedule = Schedule::new();
        schedule.insert_event(0, 0);
        schedule.insert_event(0, 1);
        schedule.insert_event(3, 2);
        schedule.insert_event(3, 3);
        schedule.insert_event(2, 4);
        schedule.insert_event(1, 5);

        assert_eq!(schedule.pop_next_events(), Some((0, vec![0, 1])));
        assert_eq!(schedule.pop_next_events(), Some((1, vec![5])));
        assert_eq!(schedule.pop_next_events(), Some((2, vec![4])));
        assert_eq!(schedule.pop_next_events(), Some((3, vec![2, 3])));
        assert_eq!(schedule.pop_next_events(), None);
        assert_eq!(schedule.pop_next_events(), None);
    }
}
