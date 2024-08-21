use std::io;

use crossterm::{
    cursor::{self, Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

// const HELP_MSG: &str = "Write /help or /h for available commands";

fn main() {
    run();
}

fn run() {
    // print_ascii_title();
    // println!("{}", HELP_MSG);
    if let Err(e) = main_menu() {
        eprintln!("Error: {}", e)
    }
}

/*
fn print_ascii_title() {
    println!("+---------------------------------------------------------------------------------------------------------+");
    println!("| ||||||||  ||||||  |||||    |||     |||  ||  |||   ||   ||||||   ||           |||||    ||||||    ||||||  |");
    println!("|    ||     ||      ||   ||  ||||| |||||  ||  ||||  ||  ||    ||  ||           ||   ||  ||   ||  ||       |");
    println!("|    ||     ||||||  |||||    ||  |||  ||  ||  || || ||  ||||||||  ||           |||||    ||||||   ||  |||  |");
    println!("|    ||     ||      ||   ||  ||       ||  ||  ||  ||||  ||    ||  ||           ||   ||  ||       ||    || |");
    println!("|    ||     ||||||  ||   ||  ||       ||  ||  ||   |||  ||    ||  ||||||       ||   ||  ||        ||||||  |");
    println!("+---------------------------------------------------------------------------------------------------------+\n");
}
*/

fn print_ascii_title_in_menu(mut out: &io::Stdout) -> io::Result<()> {
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

fn main_menu() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let menu_items = vec![
        "Load Game",
        "New Game",
        "Login (WIP)",
        "Register (WIP)",
        "Leaderboard (WIP)",
        "Settings",
        "Quit Game",
    ];
    let mut selected_index = 0;
    let start_column: u16 = 7;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        let _ = print_ascii_title_in_menu(&stdout);

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
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    execute!(
        stdout,
        LeaveAlternateScreen,
        Show,
        cursor::MoveToNextLine(1)
    )?;
    disable_raw_mode()?;

    Ok(())
}
