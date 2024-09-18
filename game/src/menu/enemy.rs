use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{
    drops::give_normal_enemy_drops,
    enemy::{Enemy, EnemyKind},
    menu::{character::menu_level_up, inventory::menu_inventory_consumable_list},
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
    let victory = menu_enemy_fight(enemy, character)?;
    execute!(stdout, Clear(ClearType::All))?;

    Ok(victory)
}

/// Returns true if the player wins the fight.
fn menu_enemy_fight(enemy: &mut Enemy, character: &mut PlayerCharacter) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let mut selected_index = 0;
    let start_column: u16 = 10;
    let mut fight_text = "Select what to do...".to_string();
    let mut action = false;
    let mut player_turn = true;

    // fully heal player at the start of fights
    character.restore_health(character.get_total_health());

    loop {
        let mut menu_items = vec!["Attack", "Consumables", "Stats", "Flee"];
        if action {
            menu_items = vec!["Continue"];
        }

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
        println!(
            "Mana: {}/{}",
            character.temp_stats.current_mana,
            character.get_total_mana()
        );
        execute!(stdout, cursor::MoveTo(0, 7))?;
        println!("Defense: {}", character.get_total_defense());

        execute!(stdout, cursor::MoveTo(0, 9))?;
        println!("{}", fight_text);

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
                    "Attack" => {
                        action = true;
                        fight_text = character.attack_enemy(enemy);
                        selected_index = 0;
                        execute!(stdout, Clear(ClearType::All))?;
                    }
                    "Consumables" => {
                        let text = menu_inventory_consumable_list(character, true)?;
                        if !text.is_empty() {
                            action = true;
                            fight_text = text;
                            selected_index = 0;
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                    }
                    "Stats" => menu_enemy_fight_character_stats(character)?,
                    "Continue" => {
                        if enemy.is_dead() {
                            let character_level =
                                character.data.stats.general_stats.character_level;
                            match enemy.kind {
                                EnemyKind::Normal => {
                                    menu_normal_enemy_fight_victory(enemy.level, character)?;
                                }
                                _ => {}
                            }
                            if character.data.stats.general_stats.character_level > character_level
                            {
                                menu_level_up(character.data.stats.general_stats.character_level)?;
                            }
                            return Ok(true);
                        }
                        if player_turn {
                            player_turn = false;
                            fight_text = enemy.attack_player(character);
                        } else {
                            action = false;
                            player_turn = true;
                            fight_text = "Select what to do...".to_string();
                        }
                        execute!(stdout, Clear(ClearType::All))?;
                    }
                    _ => break,
                },
                _ => {}
            }
        }
    }

    Ok(false)
}

fn menu_normal_enemy_fight_victory(
    enemy_level: u32,
    character: &mut PlayerCharacter,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    let drops = give_normal_enemy_drops(character, enemy_level);

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("You defeated the enemy!");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Drops:");
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("  Gold: {}", drops.gold);
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!("  EXP: {}", drops.exp);
        execute!(stdout, cursor::MoveTo(0, 4))?;
        println!("  Item: {}", drops.equipment_item_name);
        execute!(stdout, cursor::MoveTo(0, 6))?;
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

pub fn menu_enemy_fight_player_died(character: &mut PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let current_level = character.data.stats.general_stats.character_level;
    let current_floor = character.data.stats.general_stats.current_dungeon_floor;
    if current_level
        > character
            .data
            .stats
            .general_stats
            .highest_character_level_achieved
    {
        character
            .data
            .stats
            .general_stats
            .highest_character_level_achieved = current_level
    }
    if current_floor
        > character
            .data
            .stats
            .general_stats
            .highest_dungeon_floor_achieved
    {
        character
            .data
            .stats
            .general_stats
            .highest_dungeon_floor_achieved = current_floor;
    }
    character.data.stats.general_stats.deaths += 1;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("You Died!");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Character: {}", character.data.metadata.name);
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!(
            "Level: {}",
            character.data.stats.general_stats.character_level
        );
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!(
            "Highest Level Reached (Record): {}",
            character
                .data
                .stats
                .general_stats
                .highest_character_level_achieved
        );
        execute!(stdout, cursor::MoveTo(0, 4))?;
        println!(
            "Dungeon Floor: {}",
            character.data.stats.general_stats.current_dungeon_floor
        );
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!(
            "Highest Dungeon Floor Reached (Record): {}",
            character
                .data
                .stats
                .general_stats
                .highest_dungeon_floor_achieved
        );
        execute!(stdout, cursor::MoveTo(0, 6))?;
        println!("Deaths: {}", character.data.stats.general_stats.deaths);
        execute!(stdout, cursor::MoveTo(0, 8))?;
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

fn menu_enemy_fight_character_stats(character: &PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Combat Stats");
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!(
            "  Health: {}/{}",
            character.temp_stats.current_health,
            character.get_total_health()
        );
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!(
            "  Mana: {}/{}",
            character.temp_stats.current_mana,
            character.get_total_mana()
        );
        execute!(stdout, cursor::MoveTo(0, 4))?;
        println!("  Defense: {}", character.get_total_defense());
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!("  Damage: {}", character.get_total_damage());
        execute!(stdout, cursor::MoveTo(0, 6))?;
        println!(
            "  Critical Damage Multiplier: {:.2}",
            character.get_total_crit_damage_multiplier()
        );
        execute!(stdout, cursor::MoveTo(0, 7))?;
        println!(
            "  Critical Hit Rate: {:.2} ({:.2}%)",
            character.get_total_crit_hit_rate(),
            character.get_total_crit_hit_rate() * 100.0
        );

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
