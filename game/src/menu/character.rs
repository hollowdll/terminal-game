use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{
    menu::{
        equipment::menu_equipment, inventory::menu_inventory, skill::menu_skill,
        stats::menu_character_stats,
    },
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
        "Skill",
        "Return to main menu",
    ];
    let mut selected_index = 0;
    let start_column: u16 = 2;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("(Esc) Close Menu");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Menu");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
                        "Stats" => {
                            menu_character_stats(&character)?;
                        }
                        "Inventory" => {
                            menu_inventory(character, false)?;
                        }
                        "Equipment" => {
                            menu_equipment(character)?;
                        }
                        "Skill" => {
                            menu_skill(character)?;
                        }
                        "Return to main menu" => {
                            let confirm = menu_confirm_return_to_main_menu()?;
                            if confirm {
                                return Ok(true);
                            }
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                        _ => break,
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(false)
}

fn menu_confirm_return_to_main_menu() -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["No", "Yes"];
    let mut selected_index = 0;
    let start_column: u16 = 2;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Return back to the game main menu?");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Progress in the current dungeon floor will be lost.");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
    }

    match menu_items[selected_index] {
        "Yes" => return Ok(true),
        _ => {}
    }

    Ok(false)
}

pub fn menu_level_up(new_level: u32) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("You Leveled Up!");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("New Level: {}", new_level);
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("Some stats have been increased");
        execute!(stdout, cursor::MoveTo(0, 4))?;
        println!("> Continue");

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Enter => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}
