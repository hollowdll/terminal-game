use crossterm::{
    cursor::{self, Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::{self, Write};

use crate::{
    config::GameConfig,
    game::{create_new_game_character, max_game_characters_reached, save_game},
    session::Player,
};

const OPTION_LOAD_GAME: &str = "Load Game";
const OPTION_NEW_GAME: &str = "New Game";
const OPTION_QUIT_GAME: &str = "Quit Game";

fn print_ascii_title(mut out: &io::Stdout) -> io::Result<()> {
    println!("+---------------------------------------------------------------------------------------------------------+");
    execute!(out, cursor::MoveTo(0, 1))?;
    println!("| ||||||||  ||||||  |||||    |||     |||  ||  |||   ||   ||||||   ||           |||||    ||||||    ||||||  |");
    execute!(out, cursor::MoveTo(0, 2))?;
    println!("|    ||     ||      ||   ||  ||||| |||||  ||  ||||  ||  ||    ||  ||           ||   ||  ||   ||  ||       |");
    execute!(out, cursor::MoveTo(0, 3))?;
    println!("|    ||     ||||||  |||||    ||  |||  ||  ||  || || ||  ||||||||  ||           |||||    ||||||   ||  |||  |");
    execute!(out, cursor::MoveTo(0, 4))?;
    println!("|    ||     ||      ||   ||  ||       ||  ||  ||  ||||  ||    ||  ||           ||   ||  ||       ||    || |");
    execute!(out, cursor::MoveTo(0, 5))?;
    println!("|    ||     ||||||  ||   ||  ||       ||  ||  ||   |||  ||    ||  ||||||       ||   ||  ||        ||||||  |");
    execute!(out, cursor::MoveTo(0, 6))?;
    println!("+---------------------------------------------------------------------------------------------------------+\n");
    execute!(out, cursor::MoveTo(0, 7))?;

    Ok(())
}

/// Returned bool is true if the menu should be rerendered.
pub fn main_menu(player: &mut Player, cfg: &GameConfig) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec![OPTION_LOAD_GAME, OPTION_NEW_GAME, OPTION_QUIT_GAME];
    let mut selected_index = 0;
    let start_column: u16 = 7;
    let mut rerender = false;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        let _ = print_ascii_title(&stdout);

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }

    match menu_items[selected_index] {
        OPTION_LOAD_GAME => {
            // execute!(stdout, LeaveAlternateScreen, Show)?;
            if let Ok(go_back) = menu_load_game() {
                if go_back {
                    rerender = true;
                }
            }
        }
        OPTION_NEW_GAME => {
            // execute!(stdout, LeaveAlternateScreen, Show)?;
            // execute!(stdout, EnterAlternateScreen, Hide)?;
            if let Ok(go_back) = menu_new_game(player, cfg) {
                if go_back {
                    rerender = true;
                }
            }
        }
        OPTION_QUIT_GAME => {}
        _ => {}
    }

    // execute!(stdout, LeaveAlternateScreen, Show)?;

    Ok(rerender)
}

/// Returns true if menu option "Back" was selected.
fn menu_load_game() -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Back"];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("No characters found");
        execute!(stdout, cursor::MoveTo(0, 1))?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }

    match menu_items[selected_index] {
        "Back" => {}
        _ => {}
    }

    // execute!(stdout, LeaveAlternateScreen, Show)?;

    Ok(true)
}

/// Returns true if menu option "Back" was selected.
fn menu_new_game(player: &mut Player, cfg: &GameConfig) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Back"];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    if !max_game_characters_reached(player, cfg) {
        // execute!(stdout, LeaveAlternateScreen, Show)?;
        match menu_create_character(player) {
            Ok(character_created) => {
                if character_created {
                    menu_tutorial()?;
                    // execute!(stdout, LeaveAlternateScreen, Show)?;
                    return Ok(false);
                } else {
                    return Ok(true);
                }
            }
            Err(e) => return Err(e),
        }
    }

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Maximum characters reached. Delete a character to create a new one.");
        execute!(stdout, cursor::MoveTo(0, 1))?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }

    match menu_items[selected_index] {
        "Back" => {}
        _ => {}
    }

    // execute!(stdout, LeaveAlternateScreen, Show)?;

    Ok(true)
}

pub fn menu_create_character(player: &mut Player) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All), Show)?;

    let menu_items = vec!["Yes", "No"];
    let mut selected_index = 0;
    let start_column: u16 = 1;
    let mut character_created = false;

    execute!(stdout, cursor::MoveTo(0, 0))?;
    println!("Create a new character");
    execute!(stdout, cursor::MoveTo(0, 1))?;
    disable_raw_mode()?;

    print!("Enter name: ");
    stdout.flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    name = name.trim().to_string();

    enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), Hide)?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Create character with the name {}?", name);
        execute!(stdout, cursor::MoveTo(0, 1))?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }

    match menu_items[selected_index] {
        "Yes" => {
            create_new_game_character(player, &name);
            save_game(player)?;
            character_created = true;
        }
        "No" => {}
        _ => {}
    }

    // execute!(stdout, LeaveAlternateScreen)?;

    Ok(character_created)
}

pub fn menu_tutorial() -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Skip tutorial"];
    let mut selected_index = 0;
    let start_column: u16 = 2;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("New character created!");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Tutorial here");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }

    match menu_items[selected_index] {
        _ => {}
    }

    // execute!(stdout, LeaveAlternateScreen, Show)?;

    Ok(true)
}
