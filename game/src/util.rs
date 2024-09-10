use chrono::{TimeZone, Utc};
use std::env;

pub fn extract_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

pub fn timestamp_to_datetime(timestamp: i64) -> String {
    Utc.timestamp_opt(timestamp, 0).unwrap().to_rfc3339()
}

/// Checks if development mode is enabled by reading
/// environment variable TERM_RPG_GAME_MODE.
/// Returns true if it has value "development".
pub fn is_dev_mode() -> bool {
    env::var("TERM_RPG_GAME_MODE")
        .unwrap_or("release".to_string())
        .eq("development")
}
