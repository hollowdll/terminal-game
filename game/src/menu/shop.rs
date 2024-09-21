use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{menu::inventory::menu_inventory, session::PlayerCharacter};

pub fn menu_shop(character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Buy Items", "Sell Items"];
    let mut selected_index = 0;
    let start_column: u16 = 2;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Shop");
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
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter => match menu_items[selected_index] {
                    "Buy Items" => {}
                    "Sell Items" => menu_inventory(character, true)?,
                    _ => break,
                },
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}
