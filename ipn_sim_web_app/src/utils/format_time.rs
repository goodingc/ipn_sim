use ipn_sim_lib::utils::TimeMetric;

pub fn format_time(time: TimeMetric, element_limit: Option<usize>) -> String {
    if time == 0 {
        return String::from("0");
    }
    let elements_iter = TIME_UNITS
        .iter()
        .fold(
            (time, Vec::with_capacity(TIME_UNITS.len())),
            |(remaining_time, mut used_units), (current_unit, current_unit_length)| {
                let current_unit_used = remaining_time / current_unit_length;
                used_units.push((current_unit.clone(), current_unit_used));
                (remaining_time % current_unit_length, used_units)
            },
        )
        .1
        .into_iter()
        .filter_map(|(suffix, count)| {
            if count == 0 {
                return None;
            }
            Some(format!("{}{}", count, suffix))
        });

    if let Some(element_limit) = element_limit {
        elements_iter.take(element_limit).collect::<Vec<_>>()
    } else {
        elements_iter.collect::<Vec<_>>()
    }
    .join(" ")
}

const TIME_UNITS: [(&str, u64); 7] = [
    ("d", ONE_DAY),
    ("h", ONE_HOUR),
    ("m", ONE_MINUTE),
    ("s", ONE_SECOND),
    ("ms", ONE_MILLISECOND),
    ("μs", ONE_MICROSECOND),
    ("ns", ONE_NANOSECOND),
];

const ONE_NANOSECOND: TimeMetric = 1;
const ONE_MICROSECOND: TimeMetric = ONE_NANOSECOND * 1_000;
const ONE_MILLISECOND: TimeMetric = ONE_MICROSECOND * 1_000;
const ONE_SECOND: TimeMetric = ONE_MILLISECOND * 1_000;
const ONE_MINUTE: TimeMetric = ONE_SECOND * 60;
const ONE_HOUR: TimeMetric = ONE_MINUTE * 60;
const ONE_DAY: TimeMetric = ONE_HOUR * 24;
