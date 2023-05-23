use std::time::Duration;

pub fn format_duration(duration: Duration) -> String {
    let mut remaining_secs = duration.as_secs();

    let years = remaining_secs / (365 * 24 * 60 * 60);
    remaining_secs %= 365 * 24 * 60 * 60;

    let months = remaining_secs / (30 * 24 * 60 * 60);
    remaining_secs %= 30 * 24 * 60 * 60;

    let weeks = remaining_secs / (7 * 24 * 60 * 60);
    remaining_secs %= 7 * 24 * 60 * 60;

    let days = remaining_secs / (24 * 60 * 60);
    remaining_secs %= 24 * 60 * 60;

    let hours = remaining_secs / (60 * 60);
    remaining_secs %= 60 * 60;

    let minutes = remaining_secs / 60;
    remaining_secs %= 60;

    let seconds = remaining_secs;

    let mut formatted_duration = String::new();

    if years > 0 {
        formatted_duration.push_str(&format!("{}y ", years));
    }

    if months > 0 {
        formatted_duration.push_str(&format!("{}m ", months));
    }

    if weeks > 0 {
        formatted_duration.push_str(&format!("{}w ", weeks));
    }

    if days > 0 {
        formatted_duration.push_str(&format!("{}d ", days));
    }

    if hours > 0 {
        formatted_duration.push_str(&format!("{}h ", hours));
    }

    if minutes > 0 {
        formatted_duration.push_str(&format!("{}m ", minutes));
    }

    if seconds > 0 || formatted_duration.is_empty() {
        formatted_duration.push_str(&format!("{}s", seconds));
    }

    formatted_duration
}

pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}
