use ipn_sim_lib::downcast_rs::__std::cmp::Ordering;
use ipn_sim_lib::utils::TimeMetric;
use itertools::Itertools;
use num_traits::{Float, Num};

#[derive(Clone)]
pub struct ValueLogger<T: PartialEq>(pub Vec<(TimeMetric, T)>);

impl<T: PartialEq> ValueLogger<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn log_value(&mut self, time: TimeMetric, value: T) {
        let should_log = if self.0.len() < 2 {
            true
        } else {
            let last = &self.0.last().unwrap().1;
            let penultimate = &self.0[self.0.len() - 2].1;
            if *last == *penultimate && *last == value {
                self.0.last_mut().unwrap().0 = time;
                false
            } else {
                true
            }
        };
        if should_log {
            self.0.push((time, value));
        }
    }

    pub fn float_max_value(&self) -> T
    where
        T: Float,
    {
        self.0
            .iter()
            .map(|(_, value)| *value)
            .fold(T::zero(), T::max)
    }

    pub fn max_value(&self) -> T
    where
        T: Num + Ord + Copy,
    {
        self.0
            .iter()
            .map(|(_, value)| *value)
            .max()
            .unwrap_or(T::zero())
    }

    pub fn combine<U: PartialEq>(
        &self,
        other: &ValueLogger<T>,
        operator: impl Fn(&T, &T) -> U,
    ) -> ValueLogger<U> {
        self.0
            .iter()
            .map(|(time, value)| (true, time, value))
            .merge_by(
                other.0.iter().map(|(time, value)| (false, time, value)),
                |(_, self_time, _), (_, other_time, _)| **self_time <= **other_time,
            )
            .fold(
                (None, None, ValueLogger::new()),
                |(mut self_value, mut other_value, mut logger), (is_self, time, value)| {
                    if is_self {
                        self_value = Some(value)
                    } else {
                        other_value = Some(value)
                    }

                    if let Some(self_value) = self_value {
                        if let Some(other_value) = other_value {
                            logger.log_value(*time, operator(self_value, other_value))
                        }
                    }

                    (self_value, other_value, logger)
                },
            )
            .2
    }
}
