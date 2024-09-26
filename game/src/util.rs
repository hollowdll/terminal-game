use chrono::{TimeZone, Utc};
use rand::{thread_rng, Rng};
use std::{env, thread, time::Duration};

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

pub fn shift_index_back(index: usize) -> usize {
    if index == 0 {
        return 0;
    }
    index - 1
}

/// Returns true if the passed rate was rolled.
/// Rate should be between 0 and 1.
pub fn is_chance_success(rate: f64) -> bool {
    thread_rng().gen_range(0.0..1.0) < rate
}

pub fn wait(millis: u64) {
    thread::sleep(Duration::from_millis(millis))
}
