use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{
    items::{get_item_display_name, get_item_purchase_value, CharacterItem, CharacterItemOwned},
    menu::inventory::{
        menu_armor_info, menu_consumable_info, menu_inventory, menu_ring_info, menu_weapon_info,
    },
    session::PlayerCharacter,
    shop::{buy_consumable, ShopItems},
    util::shift_index_back,
};

pub fn menu_shop(shop_items: &mut ShopItems, character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    let menu_items = vec!["Buy Items", "Sell Items"];
    let mut selected_index = 0;
    let start_column: u16 = 2;

    execute!(stdout, Clear(ClearType::All))?;
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
                    "Buy Items" => menu_shop_buy_items(shop_items, character)?,
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

pub fn menu_shop_buy_items(
    shop_items: &mut ShopItems,
    character: &mut PlayerCharacter,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut menu_items = vec![];
    let mut selected_index = 0;
    let start_column: u16 = 2;

    if let Some(item) = &shop_items.weapon {
        menu_items.push(CharacterItemOwned::Weapon(item.clone()));
    }
    if let Some(item) = &shop_items.armor {
        menu_items.push(CharacterItemOwned::Armor(item.clone()));
    }
    if let Some(item) = &shop_items.ring {
        menu_items.push(CharacterItemOwned::Ring(item.clone()));
    }

    for item in &shop_items.consumables {
        menu_items.push(CharacterItemOwned::Consumable(item.clone()));
    }

    execute!(stdout, Clear(ClearType::All))?;
    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back, Enter = Item Info, B = Buy Item");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Buy Items (Gold: {})", character.data.currency.gold);

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            let (name, purchase_value) = match item {
                CharacterItemOwned::Consumable(item) => (
                    get_item_display_name(CharacterItem::Consumable(item)),
                    get_item_purchase_value(&item.rarity),
                ),
                CharacterItemOwned::Weapon(item) => (
                    get_item_display_name(CharacterItem::Weapon(item)),
                    get_item_purchase_value(&item.rarity),
                ),
                CharacterItemOwned::Armor(item) => (
                    get_item_display_name(CharacterItem::Armor(item)),
                    get_item_purchase_value(&item.rarity),
                ),
                CharacterItemOwned::Ring(item) => (
                    get_item_display_name(CharacterItem::Ring(item)),
                    get_item_purchase_value(&item.rarity),
                ),
                _ => ("?Unknown?".to_string(), 0),
            };

            if i == selected_index {
                println!("> {}   [Cost: {} Gold]", name, purchase_value);
            } else {
                println!("  {}   [Cost: {} Gold]", name, purchase_value);
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
                KeyCode::Enter => match &menu_items[selected_index] {
                    CharacterItemOwned::Consumable(item) => menu_consumable_info(item, false)?,
                    CharacterItemOwned::Weapon(item) => menu_weapon_info(item, false)?,
                    CharacterItemOwned::Armor(item) => menu_armor_info(item, false)?,
                    CharacterItemOwned::Ring(item) => menu_ring_info(item, false)?,
                    _ => {}
                },
                KeyCode::Char('B') | KeyCode::Char('b') => {
                    if !menu_items.is_empty() {
                        match &menu_items[selected_index] {
                            CharacterItemOwned::Consumable(item) => {
                                buy_consumable(item, character);
                                execute!(stdout, Clear(ClearType::All))?;
                            }
                            CharacterItemOwned::Weapon(_) => {
                                if shop_items.buy_weapon(character) {
                                    menu_items.remove(selected_index);
                                    selected_index = shift_index_back(selected_index);
                                    execute!(stdout, Clear(ClearType::All))?;
                                }
                            }
                            CharacterItemOwned::Armor(_) => {
                                if shop_items.buy_armor(character) {
                                    menu_items.remove(selected_index);
                                    selected_index = shift_index_back(selected_index);
                                    execute!(stdout, Clear(ClearType::All))?;
                                }
                            }
                            CharacterItemOwned::Ring(_) => {
                                if shop_items.buy_ring(character) {
                                    menu_items.remove(selected_index);
                                    selected_index = shift_index_back(selected_index);
                                    execute!(stdout, Clear(ClearType::All))?;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}
