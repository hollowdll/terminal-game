use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{
    character::{random_exp_amount, BASE_EXP_MAX, BASE_EXP_MIN},
    currency::{random_gold_amount, BASE_GOLD_MAX, BASE_GOLD_MIN},
    enemy::{Enemy, EXP_MULTIPLIER_NORMAL_ENEMY, GOLD_MULTIPLIER_NORMAL_ENEMY},
    items::{
        generate_random_armor, generate_random_ring, generate_random_weapon, get_item_display_name,
        random_equipment_item, CharacterItem, ItemCategory, ARMOR_BASE_VALUES, RING_BASE_VALUES,
        WEAPON_BASE_VALUES,
    },
    session::PlayerCharacter,
};

/// Returns true if the player wins the fight.
pub fn menu_enemy_encounter(
    enemy: &mut Enemy,
    character: &mut PlayerCharacter,
) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Encountered enemy {}", enemy.name);
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("> Fight");

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }
    menu_enemy_fight(enemy, character)?;
    execute!(stdout, Clear(ClearType::All))?;

    Ok(true)
}

/// Returns true if the player wins the fight.
fn menu_enemy_fight(enemy: &mut Enemy, character: &mut PlayerCharacter) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Attack", "Consumables", "Stats", "Flee"];
    let mut selected_index = 0;
    let start_column: u16 = 9;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Enemy: {}", enemy.get_display_name());
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!(
            "Health: {}/{}",
            enemy.stats.current_health, enemy.stats.max_health
        );
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("Defense: {}", enemy.get_total_defense());

        execute!(stdout, cursor::MoveTo(0, 4))?;
        println!(
            "Player: {} (Level {}, EXP: {}/{})",
            character.data.metadata.name,
            character.data.stats.general_stats.character_level,
            character.data.stats.general_stats.current_exp,
            character.data.stats.general_stats.required_exp
        );
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!(
            "Health: {}/{}",
            character.temp_stats.current_health,
            character.get_total_health()
        );
        execute!(stdout, cursor::MoveTo(0, 6))?;
        println!("Defense: {}", character.get_total_defense());

        execute!(stdout, cursor::MoveTo(0, 8))?;
        println!("Select what to do...");

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
                    "Attack" => {}
                    "Consumables" => {}
                    "Stats" => {}
                    _ => break,
                },
                _ => {}
            }
        }
    }

    Ok(false)
}

fn menu_enemy_fight_victory(enemy_level: u32, character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let gold = random_gold_amount(
        BASE_GOLD_MIN,
        BASE_GOLD_MAX,
        GOLD_MULTIPLIER_NORMAL_ENEMY,
        enemy_level,
    );
    character.give_gold(gold);
    let exp = random_exp_amount(
        BASE_EXP_MIN,
        BASE_EXP_MAX,
        EXP_MULTIPLIER_NORMAL_ENEMY,
        enemy_level,
    );
    character.gain_exp(exp);

    let equipment_item_category = random_equipment_item();
    let mut item_display_name = "?Unknown?".to_string();
    match equipment_item_category {
        ItemCategory::Weapon => {
            let weapon = generate_random_weapon(WEAPON_BASE_VALUES, enemy_level);
            character.give_weapon(&weapon);
            item_display_name = get_item_display_name(CharacterItem::Weapon(&weapon));
        }
        ItemCategory::Armor => {
            let armor = generate_random_armor(ARMOR_BASE_VALUES, enemy_level);
            character.give_armor(&armor);
            item_display_name = get_item_display_name(CharacterItem::Armor(&armor));
        }
        ItemCategory::Ring => {
            let ring = generate_random_ring(RING_BASE_VALUES, enemy_level);
            character.give_ring(&ring);
            item_display_name = get_item_display_name(CharacterItem::Ring(&ring));
        }
        _ => {}
    }

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("You defeated the enemy!");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Drops:");
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("  Gold: {}", gold);
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!("  Item: {}", item_display_name);
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!("> Continue");

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Enter => {
                    break;
                }
                _ => {}
            }
        }
    }
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}
