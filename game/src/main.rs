use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use game::main_menu::main_menu;
use std::io;

// const HELP_MSG: &str = "Write /help or /h for available commands";

fn main() -> io::Result<()> {
    run()?;
    Ok(())
}

fn run() -> io::Result<()> {
    enable_raw_mode()?;
    loop {
        if let Ok(render_again) = main_menu() {
            if !render_again {
                break;
            }
        } else {
            break;
        }
    }
    disable_raw_mode()?;
    Ok(())
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
