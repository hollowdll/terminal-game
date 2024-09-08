use chrono::{TimeZone, Utc};

pub fn extract_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

pub fn timestamp_to_datetime(timestamp: i64) -> String {
    Utc.timestamp_opt(timestamp, 0).unwrap().to_rfc3339()
}
