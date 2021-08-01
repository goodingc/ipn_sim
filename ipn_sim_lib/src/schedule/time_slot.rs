use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
pub struct TimeSlot<T: Ord, E> {
    time: T,
    next_slot: Option<Box<Self>>,
    events: Vec<E>,
}

impl<T: Ord, E> TimeSlot<T, E> {
    pub fn new(time: T) -> Self {
        Self {
            time,
            next_slot: None,
            events: vec![],
        }
    }

    pub fn new_with_event(time: T, event: E) -> Self {
        let mut slot = TimeSlot::new(time);
        slot.add_event(event);
        slot
    }

    pub fn add_event(&mut self, event: E) {
        self.events.push(event);
    }

    pub fn insert_now_or_after(&mut self, time: T, event: E) {
        if self.time == time {
            self.add_event(event);
        } else {
            if let Some(next_slot) = &mut self.next_slot {
                if let Ordering::Greater = next_slot.cmp_time(&time) {
                    let old_next_slot = self
                        .next_slot
                        .replace(Box::new(TimeSlot::new_with_event(time, event)))
                        .unwrap();
                    self.next_slot
                        .as_mut()
                        .unwrap()
                        .set_next_slot(old_next_slot);
                } else {
                    next_slot.insert_now_or_after(time, event);
                }
            } else {
                self.next_slot = Some(Box::new(TimeSlot::new_with_event(time, event)));
            }
        }
    }

    pub fn cmp_time(&self, other_time: &T) -> Ordering {
        self.time.cmp(other_time)
    }

    pub fn set_next_slot(&mut self, slot: Box<Self>) {
        self.next_slot = Some(slot);
    }

    pub fn get_next_slot(&mut self) -> Option<Self> {
        self.next_slot.take().map(|boxed_slot| *boxed_slot)
    }

    pub fn into_pair(self) -> (T, Vec<E>) {
        (self.time, self.events)
    }
}

impl<T: Ord + Copy, E> TimeSlot<T, E> {
    pub fn copy_time(&self) -> T {
        self.time
    }
}

#[cfg(test)]
mod tests {
    use crate::schedule::time_slot::TimeSlot;

    #[test]
    fn test_insert_now() {
        let mut slot = TimeSlot::new_with_event(0, 0);
        slot.insert_now_or_after(0, 1);
        slot.insert_now_or_after(0, 2);

        assert_eq!(
            slot,
            TimeSlot {
                time: 0,
                next_slot: None,
                events: vec![0, 1, 2],
            }
        )
    }

    #[test]
    fn test_insert_after() {
        let mut slot = TimeSlot::new_with_event(0, 0);
        slot.insert_now_or_after(1, 1);
        slot.insert_now_or_after(2, 2);

        assert_eq!(
            slot,
            TimeSlot {
                time: 0,
                next_slot: Some(Box::new(TimeSlot {
                    time: 1,
                    next_slot: Some(Box::new(TimeSlot {
                        time: 2,
                        next_slot: None,
                        events: vec![2],
                    })),
                    events: vec![1],
                })),
                events: vec![0],
            }
        )
    }

    #[test]
    fn test_insert_between() {
        let mut slot = TimeSlot::new_with_event(0, 0);
        slot.insert_now_or_after(2, 2);
        slot.insert_now_or_after(1, 1);

        assert_eq!(
            slot,
            TimeSlot {
                time: 0,
                next_slot: Some(Box::new(TimeSlot {
                    time: 1,
                    next_slot: Some(Box::new(TimeSlot {
                        time: 2,
                        next_slot: None,
                        events: vec![2],
                    })),
                    events: vec![1],
                })),
                events: vec![0],
            }
        )
    }
}
