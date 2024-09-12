use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{
    items::{
        get_item_display_name, ArmorItem, CharacterItem, ConsumableItem, Enchantment, ItemInfo,
        RingItem, WeaponItem,
    },
    session::PlayerCharacter,
};

pub fn menu_inventory(character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Consumables", "Weapons", "Armors", "Rings", "Back"];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Inventory (Gold: {})", character.data.currency.gold);
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
                    "Consumables" => menu_inventory_consumable_list(character)?,
                    "Weapons" => menu_inventory_weapon_list(character)?,
                    "Armors" => menu_inventory_armor_list(character)?,
                    "Rings" => menu_inventory_ring_list(character)?,
                    _ => break,
                },
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn menu_inventory_consumable_list(character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let mut menu_items = Vec::new();
    let mut selected_index = 0;
    let start_column: u16 = 2;

    for (_, item) in &character.data.inventory.consumables {
        menu_items.push(item.clone());
    }

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back, Enter = Item Info, U = Use Item, D = Delete Item");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Consumables");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        if menu_items.is_empty() {
            println!("  No consumables in inventory");
        }

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!(
                    "> {} x{}",
                    get_item_display_name(CharacterItem::Consumable(&item)),
                    item.amount_in_inventory
                );
            } else {
                println!(
                    "  {} x{}",
                    get_item_display_name(CharacterItem::Consumable(&item)),
                    item.amount_in_inventory
                );
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if !menu_items.is_empty() && selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if !menu_items.is_empty() && selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter => {
                    if !menu_items.is_empty() {
                        menu_consumable_info(&menu_items[selected_index])?;
                    }
                }
                KeyCode::Char('D') | KeyCode::Char('d') => {
                    if !menu_items.is_empty() {
                        let deleted_all =
                            menu_delete_consumable(character, &mut menu_items, selected_index)?;
                        if deleted_all {
                            selected_index = 0;
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

/// Returns true if the item was removed completely (amount in inventory 0 after deletion).
pub fn menu_delete_consumable(
    character: &mut PlayerCharacter,
    menu_items: &mut Vec<ConsumableItem>,
    selected_index: usize,
) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let selected_item = &mut menu_items[selected_index];
    let mut selected_amount: u32 = 1;
    let display_name = &get_item_display_name(CharacterItem::Consumable(&selected_item));
    let mut deleted_all = false;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back, Enter = Delete, Arrow Left = Decrease amount, Arrow Right = Increase amount");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Delete item {}", display_name);
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("Specify the amount to delete:");
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!("< x{} >", selected_amount);
        execute!(stdout, cursor::MoveTo(0, 4))?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Left => {
                    if selected_amount > 1 {
                        selected_amount -= 1;
                    }
                }
                KeyCode::Right => {
                    if selected_item.amount_in_inventory > selected_amount {
                        selected_amount += 1;
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter => {
                    if selected_amount == selected_item.amount_in_inventory {
                        if character.delete_consumable(display_name) {
                            menu_items.remove(selected_index);
                            deleted_all = true;
                        }
                    } else if selected_amount < selected_item.amount_in_inventory {
                        if character
                            .decrease_consumable_inventory_amount(display_name, selected_amount)
                        {
                            selected_item.amount_in_inventory -= selected_amount;
                        }
                    }
                    break;
                }
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(deleted_all)
}

pub fn menu_inventory_weapon_list(character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let mut menu_items = Vec::new();
    let mut selected_index = 0;
    let start_column: u16 = 2;

    for (_, item) in &character.data.inventory.weapons {
        menu_items.push(item.clone());
    }

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back, Enter = Item Info, E = Equip Item, D = Delete Item");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Weapons");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        if menu_items.is_empty() {
            println!("  No weapons in inventory");
        }

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", &get_item_display_name(CharacterItem::Weapon(&item)));
            } else {
                println!("  {}", &get_item_display_name(CharacterItem::Weapon(&item)));
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if !menu_items.is_empty() && selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if !menu_items.is_empty() && selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter => {
                    if !menu_items.is_empty() {
                        menu_weapon_info(&menu_items[selected_index])?;
                    }
                }
                KeyCode::Char('D') | KeyCode::Char('d') => {
                    if !menu_items.is_empty() {
                        let selected_item = &menu_items[selected_index];
                        let delete = menu_confirm_item_deletion(&get_item_display_name(
                            CharacterItem::Weapon(selected_item),
                        ))?;
                        if delete {
                            if character.delete_weapon(&selected_item.id) {
                                menu_items.remove(selected_index);
                                selected_index = 0;
                            }
                        }
                        execute!(stdout, Clear(ClearType::All))?;
                    }
                }
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn menu_inventory_armor_list(character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let mut menu_items = Vec::new();
    let mut selected_index = 0;
    let start_column: u16 = 2;

    for (_, item) in &character.data.inventory.armors {
        menu_items.push(item.clone());
    }

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back, Enter = Item Info, E = Equip Item, D = Delete Item");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Armors");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", &get_item_display_name(CharacterItem::Armor(&item)));
            } else {
                println!("  {}", &get_item_display_name(CharacterItem::Armor(&item)));
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if !menu_items.is_empty() && selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if !menu_items.is_empty() && selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter => {
                    if !menu_items.is_empty() {
                        menu_armor_info(&menu_items[selected_index])?;
                    }
                }
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn menu_inventory_ring_list(character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let mut menu_items = Vec::new();
    let mut selected_index = 0;
    let start_column: u16 = 2;

    for (_, item) in &character.data.inventory.rings {
        menu_items.push(item.clone());
    }

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back, Enter = Item Info, E = Equip Item, D = Delete Item");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Rings");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            if i == selected_index {
                println!("> {}", &get_item_display_name(CharacterItem::Ring(&item)));
            } else {
                println!("  {}", &get_item_display_name(CharacterItem::Ring(&item)));
            }
        }

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if !menu_items.is_empty() && selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if !menu_items.is_empty() && selected_index < menu_items.len() - 1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter => {
                    if !menu_items.is_empty() {
                        menu_ring_info(&menu_items[selected_index])?;
                    }
                }
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn menu_consumable_info(item: &ConsumableItem) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Item Info");
        let start_column = display_item_basic_info(&item.info, 2)?;

        println!("  Rarity: {:?}", item.rarity);
        execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
        println!("  Effect: {}", item.effect);
        execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
        println!("  Amount in inventory: {}", item.amount_in_inventory);
        execute!(stdout, cursor::MoveTo(0, start_column + 3))?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn menu_weapon_info(item: &WeaponItem) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Item Info");
        let start_column = display_item_basic_info(&item.info, 2)?;

        println!("  Level: {}", item.level);
        execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
        println!("  Rarity: {:?}", item.rarity);
        execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
        println!("  Damage: {}", item.damage);
        execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
        println!("  Critical Hit Rate: {:.2}", item.crit_hit_rate);
        let _ = display_item_enchantments(&item.enchantments, start_column + 4);

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn menu_armor_info(item: &ArmorItem) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Item Info");
        let start_column = display_item_basic_info(&item.info, 2)?;

        println!("  Level: {}", item.level);
        execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
        println!("  Rarity: {:?}", item.rarity);
        execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
        println!("  Health: {}", item.health);
        execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
        println!("  Defense: {}", item.defense);
        let _ = display_item_enchantments(&item.enchantments, start_column + 4);

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn menu_ring_info(item: &RingItem) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Item Info");
        let start_column = display_item_basic_info(&item.info, 2)?;

        println!("  Level: {}", item.level);
        execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
        println!("  Rarity: {:?}", item.rarity);
        execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
        println!("  Mana: {}", item.mana);
        let _ = display_item_enchantments(&item.enchantments, start_column + 3);

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn display_item_basic_info(info: &ItemInfo, start_column: u16) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("  Name: {}", info.name);
    execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
    println!("  Description: {}", info.description);
    execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
    println!("  Category: {:?}", info.category);
    execute!(stdout, cursor::MoveTo(0, start_column + 3))?;

    Ok(start_column + 3)
}

pub fn display_item_enchantments(
    enchantments: &Vec<Enchantment>,
    start_column: u16,
) -> io::Result<u16> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, start_column))?;
    println!("  Enchantments:");
    let mut current_column = start_column + 1;

    if enchantments.is_empty() {
        execute!(stdout, cursor::MoveTo(0, current_column))?;
        println!("    No Enchantments");
        return Ok(current_column + 1);
    }

    for enchantment in enchantments {
        execute!(stdout, cursor::MoveTo(0, current_column))?;
        match enchantment {
            Enchantment::Damage(val) => {
                println!("    Damage: +{}", val);
            }
            Enchantment::CritHitRate(val) => {
                println!("    Critical Hit Rate: +{:.2}", val);
            }
            Enchantment::Health(val) => {
                println!("    Health: +{}", val);
            }
            Enchantment::Defense(val) => {
                println!("    Defense: +{}", val);
            }
            Enchantment::Mana(val) => {
                println!("    Mana: +{}", val);
            }
            _ => println!("?Unknown?"),
        }
        current_column += 1;
    }

    Ok(current_column)
}

fn menu_confirm_item_deletion(item_name: &str) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["No", "Yes"];
    let mut selected_index = 0;
    let start_column: u16 = 1;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!(
            "Delete item {}? It cannot be restored once deleted.",
            item_name
        );
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
        "Yes" => return Ok(true),
        _ => {}
    }

    Ok(false)
}
