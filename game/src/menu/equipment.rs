use crate::{
    items::{get_item_display_name, CharacterItem},
    menu::inventory::{menu_armor_info, menu_ring_info, menu_weapon_info},
    session::PlayerCharacter,
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

pub fn menu_equipment(character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items_num = 3;
    let mut selected_index = 0;
    let start_column: u16 = 2;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back, Enter = Item Info, U = Unequip Item");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Equipment");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        let weapon_text = match &character.data.equipment.weapon {
            Some(weapon) => &get_item_display_name(CharacterItem::Weapon(weapon)),
            None => "Not equipped",
        };
        let armor_text = match &character.data.equipment.armor {
            Some(armor) => &get_item_display_name(CharacterItem::Armor(armor)),
            None => "Not equipped",
        };
        let ring_text = match &character.data.equipment.ring {
            Some(ring) => &get_item_display_name(CharacterItem::Ring(ring)),
            None => "Not equipped",
        };

        for i in 0..menu_items_num {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                match i {
                    0 => println!("> Weapon: {}", weapon_text),
                    1 => println!("> Armor: {}", armor_text),
                    2 => println!("> Ring: {}", ring_text),
                    _ => println!("> ?Unknown?"),
                }
            } else {
                match i {
                    0 => println!("  Weapon: {}", weapon_text),
                    1 => println!("  Armor: {}", armor_text),
                    2 => println!("  Ring: {}", ring_text),
                    _ => println!("  ?Unknown?"),
                }
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
                    if selected_index < menu_items_num - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter => match selected_index {
                    0 => {
                        if let Some(weapon) = &character.data.equipment.weapon {
                            menu_weapon_info(weapon)?;
                        }
                    }
                    1 => {
                        if let Some(armor) = &character.data.equipment.armor {
                            menu_armor_info(armor)?;
                        }
                    }
                    2 => {
                        if let Some(ring) = &character.data.equipment.ring {
                            menu_ring_info(ring)?;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}
