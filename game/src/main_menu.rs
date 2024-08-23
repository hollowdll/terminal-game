use crossterm::{
    cursor::{self, Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

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
pub fn main_menu() -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

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
            execute!(stdout, LeaveAlternateScreen, Show)?;
            if let Ok(pressed_back) = menu_load_game() {
                if pressed_back {
                    rerender = true;
                }
            }
        }
        OPTION_NEW_GAME => {
            execute!(stdout, LeaveAlternateScreen, Show)?;
            if let Ok(pressed_back) = menu_new_game() {
                if pressed_back {
                    rerender = true;
                }
            }
        }
        OPTION_QUIT_GAME => {}
        _ => {}
    }

    execute!(stdout, LeaveAlternateScreen, Show)?;

    Ok(rerender)
}

/// Returns true if menu option "Back" was pressed.
fn menu_load_game() -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

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

    execute!(stdout, LeaveAlternateScreen, Show)?;

    Ok(true)
}

fn menu_new_game() -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let menu_items = vec!["Back"];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Create a new character");
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

    execute!(stdout, LeaveAlternateScreen, Show)?;

    Ok(true)
}
