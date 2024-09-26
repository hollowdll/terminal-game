use crate::{
    items::{get_item_display_name, get_item_level_display, CharacterItem, ItemRarity},
    menu::inventory::{menu_armor_info, menu_ring_info, menu_weapon_info},
    session::PlayerCharacter,
    util::{reset_text_color, set_rarity_text_color},
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
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
        println!("(Esc) Back, (Enter) Item Info, (U) Unequip Item");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Equipment");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        let (weapon_name, weapon_lvl, weapon_rarity) = match &character.equipped_items.weapon {
            Some(id) => match character.data.inventory.weapons.get(id) {
                Some(weapon) => (
                    get_item_display_name(CharacterItem::Weapon(weapon)),
                    weapon.level,
                    &weapon.rarity,
                ),
                None => ("?Unknown?".to_owned(), 0, &ItemRarity::Unknown),
            },
            None => ("Not equipped".to_owned(), 0, &ItemRarity::Unknown),
        };
        let (armor_name, armor_lvl, armor_rarity) = match &character.equipped_items.armor {
            Some(id) => match character.data.inventory.armors.get(id) {
                Some(armor) => (
                    get_item_display_name(CharacterItem::Armor(armor)),
                    armor.level,
                    &armor.rarity,
                ),
                None => ("?Unknown?".to_owned(), 0, &ItemRarity::Unknown),
            },
            None => ("Not equipped".to_owned(), 0, &ItemRarity::Unknown),
        };
        let (ring_name, ring_lvl, ring_rarity) = match &character.equipped_items.ring {
            Some(id) => match character.data.inventory.rings.get(id) {
                Some(ring) => (
                    get_item_display_name(CharacterItem::Ring(ring)),
                    ring.level,
                    &ring.rarity,
                ),
                None => ("?Unknown?".to_owned(), 0, &ItemRarity::Unknown),
            },
            None => ("Not equipped".to_owned(), 0, &ItemRarity::Unknown),
        };

        for i in 0..menu_items_num {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                print!("> ");
            } else {
                print!("  ");
            }
            match i {
                0 => {
                    print!("Weapon:");
                    set_rarity_text_color(weapon_rarity)?;
                    print!(" {}", weapon_name);
                    if weapon_lvl > 0 {
                        reset_text_color()?;
                        print!(" {}", get_item_level_display(weapon_lvl));
                    }
                }
                1 => {
                    print!("Armor:");
                    set_rarity_text_color(armor_rarity)?;
                    print!(" {}", armor_name);
                    if armor_lvl > 0 {
                        reset_text_color()?;
                        print!(" {}", get_item_level_display(armor_lvl));
                    }
                }
                2 => {
                    print!("Ring:");
                    set_rarity_text_color(ring_rarity)?;
                    print!(" {}", ring_name);
                    if ring_lvl > 0 {
                        reset_text_color()?;
                        print!(" {}", get_item_level_display(ring_lvl));
                    }
                }
                _ => println!("?Unknown?"),
            }
            reset_text_color()?;
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
                        if selected_index < menu_items_num - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Enter => match selected_index {
                        0 => {
                            if let Some(id) = &character.equipped_items.weapon {
                                if let Some(weapon) = character.data.inventory.weapons.get(id) {
                                    menu_weapon_info(weapon, false)?;
                                }
                            }
                        }
                        1 => {
                            if let Some(id) = &character.equipped_items.armor {
                                if let Some(armor) = character.data.inventory.armors.get(id) {
                                    menu_armor_info(armor, false)?;
                                }
                            }
                        }
                        2 => {
                            if let Some(id) = &character.equipped_items.ring {
                                if let Some(ring) = character.data.inventory.rings.get(id) {
                                    menu_ring_info(ring, false)?;
                                }
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Char('U') | KeyCode::Char('u') => match selected_index {
                        0 => {
                            if character.unequip_weapon() {
                                execute!(stdout, Clear(ClearType::All))?;
                            }
                        }
                        1 => {
                            if character.unequip_armor() {
                                execute!(stdout, Clear(ClearType::All))?;
                            }
                        }
                        2 => {
                            if character.unequip_ring() {
                                execute!(stdout, Clear(ClearType::All))?;
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}
