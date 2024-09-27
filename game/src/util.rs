use chrono::{TimeZone, Utc};
use crossterm::{
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
};
use rand::{thread_rng, Rng};
use std::{
    env,
    io::{self},
    thread,
    time::Duration,
};

use crate::items::ItemRarity;

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

pub fn set_rarity_text_color(rarity: &ItemRarity) -> io::Result<()> {
    let mut stdout = io::stdout();
    match rarity {
        ItemRarity::Common => execute!(stdout, SetForegroundColor(Color::White))?,
        ItemRarity::Uncommon => execute!(stdout, SetForegroundColor(Color::Green))?,
        ItemRarity::Rare => execute!(stdout, SetForegroundColor(Color::Blue))?,
        ItemRarity::Epic => execute!(stdout, SetForegroundColor(Color::DarkCyan))?,
        ItemRarity::Legendary => execute!(stdout, SetForegroundColor(Color::Yellow))?,
        ItemRarity::Mythical => execute!(stdout, SetForegroundColor(Color::Red))?,
        _ => execute!(stdout, SetForegroundColor(Color::Reset))?,
    }
    Ok(())
}

pub fn reset_text_color() -> io::Result<()> {
    execute!(io::stdout(), SetForegroundColor(Color::Reset))
}

pub fn reset_background_color() -> io::Result<()> {
    execute!(io::stdout(), SetBackgroundColor(Color::Reset))
}

pub fn display_health_bar(percentage: u16, current_health: u32, max_health: u32) -> io::Result<()> {
    let mut stdout = io::stdout();
    let bar_length = 25;
    let mut filled_length = (percentage as usize * bar_length) / 100;
    let mut empty_length = bar_length - filled_length;

    if percentage == 0 {
        filled_length = 0;
        empty_length = bar_length;
    } else if percentage == 1 {
        filled_length = 1;
        empty_length = bar_length - 1;
    }

    print!("Health: {}/{} [", current_health, max_health);
    execute!(stdout, SetForegroundColor(Color::Red))?;
    print!("{}", "=".repeat(filled_length));
    execute!(
        stdout,
        SetForegroundColor(Color::Rgb {
            r: 50,
            g: 50,
            b: 50
        })
    )?;
    print!("{}", "=".repeat(empty_length));
    reset_text_color()?;
    print!("]");

    Ok(())
}

pub fn display_mana_bar(percentage: u16, current_mana: u32, max_mana: u32) -> io::Result<()> {
    let mut stdout = io::stdout();
    let bar_length = 25;
    let mut filled_length = (percentage as usize * bar_length) / 100;
    let mut empty_length = bar_length - filled_length;

    if percentage == 0 {
        filled_length = 0;
        empty_length = bar_length;
    } else if percentage == 1 {
        filled_length = 1;
        empty_length = bar_length - 1;
    }

    print!("Mana: {}/{} [", current_mana, max_mana);
    execute!(stdout, SetForegroundColor(Color::DarkCyan))?;
    print!("{}", "=".repeat(filled_length));
    execute!(
        stdout,
        SetForegroundColor(Color::Rgb {
            r: 50,
            g: 50,
            b: 50
        })
    )?;
    print!("{}", "=".repeat(empty_length));
    reset_text_color()?;
    print!("]");

    Ok(())
}
