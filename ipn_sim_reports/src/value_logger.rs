use ipn_sim_lib::utils::TimeMetric;
use num_traits::{Num, Float};
use ipn_sim_lib::downcast_rs::__std::cmp::Ordering;
use itertools::Itertools;

#[derive(Clone)]
pub struct ValueLogger<T: PartialEq> {
    pub value: T,
    pub history: Vec<(TimeMetric, T)>,
    discrete: bool,
}

impl<T: PartialEq> ValueLogger<T> {
    pub fn new(initial_value: T, discrete: bool) -> Self {
        Self {
            value: initial_value,
            history: vec![],
            discrete,
        }
    }

    pub fn log_value(&mut self, time: TimeMetric, value: T) where T: Copy {
        let mut should_log = if self.history.len() < 2 {
            true
        } else {
            let last = &self.history.last().unwrap().1;
            let penultimate = &self.history[self.history.len() - 2].1;
            if *last == *penultimate && *last == value {
                self.history.last_mut().unwrap().0 = time;
                false
            } else {
                true
            }
        };
        if should_log {
            if self.discrete {
                self.history.push((time, self.value));
            }
            self.value = value;
            self.history.push((time, value));
        }
    }

    pub fn float_max_value(&self) -> T where T: Float {
        self.history
            .iter()
            .map(|(_, value)| *value)
            .fold(T::zero(), T::max)
    }

    pub fn max_value(&self) -> T where T: Num + Ord + Copy {
        self.history
            .iter()
            .map(|(_, value)| *value)
            .max()
            .unwrap_or(T::zero())
    }

    pub fn combine<U: PartialEq + Copy>(&self, other: &ValueLogger<T>, operator: impl Fn(&T, &T) -> U, result_logger: ValueLogger<U>) -> ValueLogger<U> {
        self.history
            .iter()
            .map(|(time, value)| (true, time, value))
            .merge_by(
                other.history
                    .iter()
                    .map(|(time, value)| (false, time, value)),
                |(_, self_time, _), (_, other_time, _)| {
                    **self_time <= **other_time
                },
            ).fold(
            (None, None, result_logger),
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
            }).2
    }
}