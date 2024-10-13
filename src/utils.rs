use std::time::Duration;

pub fn parse_interval(interval_str: &str) -> Option<Duration> {
    let num = interval_str[..interval_str.len() - 1].parse::<u64>().ok()?;
    let unit = interval_str.chars().last()?;

    match unit {
        'm' => Some(Duration::from_secs(num * 60)),
        'h' => Some(Duration::from_secs(num * 60 * 60)),
        'd' => Some(Duration::from_secs(num * 60 * 60 * 24)),
        _ => None,
    }
}
