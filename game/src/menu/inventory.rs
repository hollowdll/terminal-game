use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{
    items::{
        get_item_display_name, get_item_sell_value, ArmorItem, CharacterItem, ConsumableItem,
        Enchantment, ItemInfo, RingItem, WeaponItem,
    },
    session::PlayerCharacter,
    shop::{sell_armor, sell_consumable, sell_ring, sell_weapon},
    util::{reset_text_color, set_rarity_text_color, shift_index_back},
};

pub fn menu_inventory(character: &mut PlayerCharacter, sell_items: bool) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Consumables", "Weapons", "Armors", "Rings"];
    let mut selected_index = 0;
    let mut start_column: u16 = 2;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("(Esc) Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Inventory (Gold: {})", character.data.currency.gold);
        execute!(stdout, cursor::MoveTo(0, 2))?;
        if sell_items {
            println!("Sell Items");
            start_column = 3;
        }

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
                        "Consumables" => {
                            let _ = menu_inventory_consumable_list(character, false, sell_items)?;
                        }
                        "Weapons" => menu_inventory_weapon_list(character, sell_items)?,
                        "Armors" => menu_inventory_armor_list(character, sell_items)?,
                        "Rings" => menu_inventory_ring_list(character, sell_items)?,
                        _ => break,
                    },
                    _ => {}
                }
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

/// Returns text telling what the item did if it was used.
pub fn menu_inventory_consumable_list(
    character: &mut PlayerCharacter,
    in_fight: bool,
    sell_items: bool,
) -> io::Result<(String, String)> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let mut menu_items = Vec::new();
    let mut selected_index = 0;
    let start_column: u16 = 2;
    let mut event_text = "".to_string();
    let mut effect_text = "".to_string();

    for (_, item) in &character.data.inventory.consumables {
        menu_items.push(item.clone());
    }

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        if in_fight {
            println!("(Esc) Back, (Enter) Item Info, (U) Use Item");
        } else if sell_items {
            println!("(Esc) Back, (Enter) Item Info, (S) Sell Item");
        } else {
            println!("(Esc) Back, (Enter) Item Info, (D) Delete Item");
        }
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Consumables");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        if menu_items.is_empty() {
            println!("  No consumables in inventory");
        }

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            let name = get_item_display_name(CharacterItem::Consumable(&item));
            if i == selected_index {
                println!("> {} x{}", name, item.amount_in_inventory);
            } else {
                println!("  {} x{}", name, item.amount_in_inventory);
            }
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
                            menu_consumable_info(&menu_items[selected_index], sell_items)?;
                        }
                    }
                    KeyCode::Char('U') | KeyCode::Char('u') => {
                        if in_fight && !menu_items.is_empty() {
                            let selected_item = &menu_items[selected_index];
                            (event_text, effect_text) = selected_item.use_item(character);
                            break;
                        }
                    }
                    KeyCode::Char('D') | KeyCode::Char('d') => {
                        if !in_fight && !sell_items && !menu_items.is_empty() {
                            let deleted_all = menu_delete_consumable(
                                character,
                                &mut menu_items,
                                selected_index,
                                false,
                            )?;
                            if deleted_all {
                                selected_index = shift_index_back(selected_index);
                            }
                        }
                    }
                    KeyCode::Char('S') | KeyCode::Char('s') => {
                        if sell_items && !menu_items.is_empty() {
                            let deleted_all = menu_delete_consumable(
                                character,
                                &mut menu_items,
                                selected_index,
                                true,
                            )?;
                            if deleted_all {
                                selected_index = shift_index_back(selected_index);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok((event_text, effect_text))
}

/// Returns true if the item was removed completely (amount in inventory 0 after deletion).
pub fn menu_delete_consumable(
    character: &mut PlayerCharacter,
    menu_items: &mut Vec<ConsumableItem>,
    selected_index: usize,
    sell_item: bool,
) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    let selected_item = &mut menu_items[selected_index];
    let mut selected_amount: u32 = 1;
    let display_name = &get_item_display_name(CharacterItem::Consumable(&selected_item));
    let mut deleted_all = false;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        if sell_item {
            println!("(Esc) Back, (Enter) Sell, (<-) Decrease amount, (->) Increase amount");
        } else {
            println!("(Esc) Back, (Enter) Delete, (<-) Decrease amount, (->) Increase amount");
        }
        execute!(stdout, cursor::MoveTo(0, 1))?;
        if sell_item {
            println!("Sell item {}", display_name);
        } else {
            println!("Delete item {}", display_name);
        }
        execute!(stdout, cursor::MoveTo(0, 2))?;
        if sell_item {
            println!("Specify the amount to sell:");
        } else {
            println!("Specify the amount to delete:");
        }
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!("< x{} >", selected_amount);
        execute!(stdout, cursor::MoveTo(0, 4))?;

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
                            if sell_item {
                                let gold =
                                    sell_consumable(selected_item, selected_amount, character);
                                if gold != 0 {
                                    menu_items.remove(selected_index);
                                    deleted_all = true;
                                }
                            } else {
                                if character.delete_consumable(display_name) {
                                    menu_items.remove(selected_index);
                                    deleted_all = true;
                                }
                            }
                        } else if selected_amount < selected_item.amount_in_inventory {
                            if sell_item {
                                let gold =
                                    sell_consumable(selected_item, selected_amount, character);
                                if gold != 0 {
                                    selected_item.amount_in_inventory -= selected_amount;
                                }
                            } else {
                                if character.decrease_consumable_inventory_amount(
                                    display_name,
                                    selected_amount,
                                ) {
                                    selected_item.amount_in_inventory -= selected_amount;
                                }
                            }
                        }
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(deleted_all)
}

pub fn menu_inventory_weapon_list(
    character: &mut PlayerCharacter,
    sell_items: bool,
) -> io::Result<()> {
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
        if sell_items {
            println!("(Esc) Back, (Enter) Item Info, (S) Sell Item");
        } else {
            println!("(Esc) Back, (Enter) Item Info, (E) Equip Item, (D) Delete Item");
        }
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Weapons");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        if menu_items.is_empty() {
            println!("  No weapons in inventory");
        }

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            let display_name = &get_item_display_name(CharacterItem::Weapon(&item));
            if i == selected_index {
                if item.is_equipped(&character) {
                    print!("> ");
                    set_rarity_text_color(&item.rarity)?;
                    print!("{}", display_name);
                    reset_text_color()?;
                    print!(" [Equipped]");
                } else {
                    print!("> ");
                    set_rarity_text_color(&item.rarity)?;
                    print!("{}", display_name);
                }
            } else {
                if item.is_equipped(&character) {
                    set_rarity_text_color(&item.rarity)?;
                    print!("  {}", display_name);
                    reset_text_color()?;
                    print!(" [Equipped]");
                } else {
                    set_rarity_text_color(&item.rarity)?;
                    println!("  {}", display_name);
                }
            }
            reset_text_color()?;
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
                            menu_weapon_info(&menu_items[selected_index], sell_items)?;
                        }
                    }
                    KeyCode::Char('D') | KeyCode::Char('d') => {
                        if !menu_items.is_empty() && !sell_items {
                            let selected_item = &menu_items[selected_index];
                            let delete = menu_confirm_item_deletion(&get_item_display_name(
                                CharacterItem::Weapon(selected_item),
                            ))?;
                            if delete {
                                if character.delete_weapon(&selected_item.id) {
                                    menu_items.remove(selected_index);
                                    selected_index = shift_index_back(selected_index);
                                }
                            }
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                    }
                    KeyCode::Char('E') | KeyCode::Char('e') => {
                        if !menu_items.is_empty() && !sell_items {
                            if character.equip_weapon(&menu_items[selected_index].id) {
                                execute!(stdout, Clear(ClearType::All))?;
                            }
                        }
                    }
                    KeyCode::Char('S') | KeyCode::Char('s') => {
                        if !menu_items.is_empty() && sell_items {
                            if sell_weapon(&menu_items[selected_index], character) != 0 {
                                menu_items.remove(selected_index);
                                selected_index = shift_index_back(selected_index);
                                execute!(stdout, Clear(ClearType::All))?;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn menu_inventory_armor_list(
    character: &mut PlayerCharacter,
    sell_items: bool,
) -> io::Result<()> {
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
        if sell_items {
            println!("(Esc) Back, (Enter) Item Info, (S) Sell Item");
        } else {
            println!("(Esc) Back, (Enter) Item Info, (E) Equip Item, (D) Delete Item");
        }
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Armors");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        if menu_items.is_empty() {
            println!("  No armors in inventory");
        }

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            let display_name = &get_item_display_name(CharacterItem::Armor(&item));
            if i == selected_index {
                if item.is_equipped(&character) {
                    println!("> {} [Equipped]", display_name);
                } else {
                    println!("> {}", display_name);
                }
            } else {
                if item.is_equipped(&character) {
                    println!("  {} [Equipped]", display_name);
                } else {
                    println!("  {}", display_name);
                }
            }
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
                            menu_armor_info(&menu_items[selected_index], sell_items)?;
                        }
                    }
                    KeyCode::Char('D') | KeyCode::Char('d') => {
                        if !menu_items.is_empty() && !sell_items {
                            let selected_item = &menu_items[selected_index];
                            let delete = menu_confirm_item_deletion(&get_item_display_name(
                                CharacterItem::Armor(selected_item),
                            ))?;
                            if delete {
                                if character.delete_armor(&selected_item.id) {
                                    menu_items.remove(selected_index);
                                    selected_index = shift_index_back(selected_index);
                                }
                            }
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                    }
                    KeyCode::Char('E') | KeyCode::Char('e') => {
                        if !menu_items.is_empty() && !sell_items {
                            if character.equip_armor(&menu_items[selected_index].id) {
                                execute!(stdout, Clear(ClearType::All))?;
                            }
                        }
                    }
                    KeyCode::Char('S') | KeyCode::Char('s') => {
                        if !menu_items.is_empty() && sell_items {
                            if sell_armor(&menu_items[selected_index], character) != 0 {
                                menu_items.remove(selected_index);
                                selected_index = shift_index_back(selected_index);
                                execute!(stdout, Clear(ClearType::All))?;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn menu_inventory_ring_list(
    character: &mut PlayerCharacter,
    sell_items: bool,
) -> io::Result<()> {
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
        if sell_items {
            println!("(Esc) Back, (Enter) Item Info, (S) Sell Item");
        } else {
            println!("(Esc) Back, (Enter) Item Info, (E) Equip Item, (D) Delete Item");
        }
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Rings");
        execute!(stdout, cursor::MoveTo(0, 2))?;

        if menu_items.is_empty() {
            println!("  No rings in inventory");
        }

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + start_column))?;
            let display_name = &get_item_display_name(CharacterItem::Ring(&item));
            if i == selected_index {
                if item.is_equipped(&character) {
                    println!("> {} [Equipped]", display_name);
                } else {
                    println!("> {}", display_name);
                }
            } else {
                if item.is_equipped(&character) {
                    println!("  {} [Equipped]", display_name);
                } else {
                    println!("  {}", display_name);
                }
            }
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
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
                            menu_ring_info(&menu_items[selected_index], sell_items)?;
                        }
                    }
                    KeyCode::Char('D') | KeyCode::Char('d') => {
                        if !menu_items.is_empty() && !sell_items {
                            let selected_item = &menu_items[selected_index];
                            let delete = menu_confirm_item_deletion(&get_item_display_name(
                                CharacterItem::Ring(selected_item),
                            ))?;
                            if delete {
                                if character.delete_ring(&selected_item.id) {
                                    menu_items.remove(selected_index);
                                    selected_index = shift_index_back(selected_index);
                                }
                            }
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                    }
                    KeyCode::Char('E') | KeyCode::Char('e') => {
                        if !menu_items.is_empty() && !sell_items {
                            let selected_item = &menu_items[selected_index];
                            if character.equip_ring(&selected_item.id) {
                                execute!(stdout, Clear(ClearType::All))?;
                            }
                        }
                    }
                    KeyCode::Char('S') | KeyCode::Char('s') => {
                        if !menu_items.is_empty() && sell_items {
                            if sell_ring(&menu_items[selected_index], character) != 0 {
                                menu_items.remove(selected_index);
                                selected_index = shift_index_back(selected_index);
                                execute!(stdout, Clear(ClearType::All))?;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}

pub fn menu_consumable_info(item: &ConsumableItem, sell_item: bool) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("(Esc) Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Item Info");
        let start_column = display_item_basic_info(&item.info, 2)?;

        println!("  Rarity: {:?}", item.rarity);
        execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
        println!("  Effect: {}", item.effect);
        execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
        println!("  Amount in Inventory: {}", item.amount_in_inventory);
        execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
        if sell_item {
            println!("  Sell Value: {} Gold", get_item_sell_value(&item.rarity));
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Esc => {
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

pub fn menu_weapon_info(item: &WeaponItem, sell_item: bool) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("(Esc) Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Item Info");
        let start_column = display_item_basic_info(&item.info, 2)?;

        println!("  Level: {}", item.level);
        execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
        println!("  Rarity: {:?}", item.rarity);
        execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
        println!("  Damage: {}", item.stats.damage);
        execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
        println!("  Critical Hit Rate: {:.2}", item.stats.crit_hit_rate);
        let column = display_item_enchantments(&item.enchantments, start_column + 4)?;
        if sell_item {
            execute!(stdout, cursor::MoveTo(0, column))?;
            println!("  Sell Value: {} Gold", get_item_sell_value(&item.rarity));
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Esc => {
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

pub fn menu_armor_info(item: &ArmorItem, sell_item: bool) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("(Esc) Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Item Info");
        let start_column = display_item_basic_info(&item.info, 2)?;

        println!("  Level: {}", item.level);
        execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
        println!("  Rarity: {:?}", item.rarity);
        execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
        println!("  Health: {}", item.stats.health);
        execute!(stdout, cursor::MoveTo(0, start_column + 3))?;
        println!("  Defense: {}", item.stats.defense);
        let column = display_item_enchantments(&item.enchantments, start_column + 4)?;
        if sell_item {
            execute!(stdout, cursor::MoveTo(0, column))?;
            println!("  Sell Value: {} Gold", get_item_sell_value(&item.rarity));
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Esc => {
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

pub fn menu_ring_info(item: &RingItem, sell_item: bool) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("(Esc) Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Item Info");
        let start_column = display_item_basic_info(&item.info, 2)?;

        println!("  Level: {}", item.level);
        execute!(stdout, cursor::MoveTo(0, start_column + 1))?;
        println!("  Rarity: {:?}", item.rarity);
        execute!(stdout, cursor::MoveTo(0, start_column + 2))?;
        println!("  Mana: {}", item.stats.mana);
        let column = display_item_enchantments(&item.enchantments, start_column + 3)?;
        if sell_item {
            execute!(stdout, cursor::MoveTo(0, column))?;
            println!("  Sell Value: {} Gold", get_item_sell_value(&item.rarity));
        }

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == KeyEventKind::Press {
                match code {
                    KeyCode::Esc => {
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
