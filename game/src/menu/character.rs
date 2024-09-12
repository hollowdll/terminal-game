use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{
    menu::{inventory::menu_inventory, stats::menu_character_stats},
    session::PlayerCharacter,
};

/// Returns true if should go back to main menu.
pub fn menu_character(character: &mut PlayerCharacter) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec![
        "Stats",
        "Inventory",
        "Equipment",
        "Return to main menu",
        "Back",
    ];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Menu");
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
                KeyCode::Enter => match menu_items[selected_index] {
                    "Stats" => {
                        menu_character_stats(&character)?;
                    }
                    "Inventory" => {
                        menu_inventory(character)?;
                    }
                    "Equipment" => {}
                    "Return to main menu" => return Ok(true),
                    _ => break,
                },
                _ => {}
            }
        }
    }

    Ok(false)
}
