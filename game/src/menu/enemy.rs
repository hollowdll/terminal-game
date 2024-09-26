use crate::{
    character::SKILL_MANA_COST,
    drops::{give_ancient_enemy_drops, give_boss_enemy_drops, give_normal_enemy_drops},
    enemy::{Enemy, EnemyKind, ENEMY_SKILL_CHANCE},
    menu::{character::menu_level_up, inventory::menu_inventory_consumable_list},
    session::PlayerCharacter,
    util::is_chance_success,
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

const DEFAULT_FIGHT_TEXT: &str = "Select what to do...";

/// Returns true if the player wins the fight.
pub fn menu_enemy_encounter(
    enemy: &mut Enemy,
    character: &mut PlayerCharacter,
) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Encountered enemy {}", enemy.get_display_name());
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("> Fight");

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
    let victory = menu_enemy_fight(enemy, character)?;
    execute!(stdout, Clear(ClearType::All))?;

    Ok(victory)
}

/// Returns true if the player wins the fight.
fn menu_enemy_fight(enemy: &mut Enemy, character: &mut PlayerCharacter) -> io::Result<bool> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let mut selected_index = 0;
    let mut fight_text = DEFAULT_FIGHT_TEXT.to_string();
    let mut effect_text = "".to_string();
    let mut action = false;
    let mut player_turn = true;
    let player_temp_stat_boosts = character.temp_stat_boosts.clone();

    // fully heal player at the start of fights
    character.restore_health(character.get_total_health());
    character.restore_mana(character.get_total_mana());

    loop {
        let mut menu_items = vec!["Attack", "Use Skill", "Consumables", "Stats"];
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
            "Player: {} (Level {} {:?}, EXP: {}/{})",
            character.data.metadata.name,
            character.data.stats.general_stats.character_level,
            character.data.metadata.class,
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

        let start_column: u16 = match action {
            true => {
                execute!(stdout, cursor::MoveTo(0, 10))?;
                println!("{}", effect_text);
                11
            }
            _ => 10,
        };
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
                    KeyCode::Enter => match menu_items[selected_index] {
                        "Attack" => {
                            action = true;
                            let (event, effect) = character.attack_enemy(enemy);
                            fight_text = event.to_string();
                            effect_text = effect;
                            selected_index = 0;
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                        "Use Skill" => {
                            if character.has_enough_mana_for_skill() {
                                action = true;
                                (fight_text, effect_text) = character.use_skill(enemy);
                                selected_index = 0;
                                execute!(stdout, Clear(ClearType::All))?;
                            } else {
                                fight_text = format!(
                                    "Not enough mana to use skill ({} required)",
                                    SKILL_MANA_COST
                                );
                            }
                        }
                        "Consumables" => {
                            let (event, effect) =
                                menu_inventory_consumable_list(character, true, false)?;
                            if !event.is_empty() && !effect.is_empty() {
                                action = true;
                                fight_text = event;
                                effect_text = effect;
                                selected_index = 0;
                                execute!(stdout, Clear(ClearType::All))?;
                            } else {
                                fight_text = DEFAULT_FIGHT_TEXT.to_string();
                                effect_text = "".to_string();
                            }
                        }
                        "Stats" => {
                            menu_enemy_fight_character_stats(character)?;
                            fight_text = DEFAULT_FIGHT_TEXT.to_string();
                            effect_text = "".to_string();
                        }
                        "Continue" => {
                            if enemy.is_dead() {
                                let character_level =
                                    character.data.stats.general_stats.character_level;
                                match enemy.kind {
                                    EnemyKind::Normal => {
                                        menu_normal_enemy_fight_victory(enemy.level, character)?;
                                    }
                                    EnemyKind::Boss => {
                                        menu_boss_enemy_fight_victory(enemy.level, character)?;
                                    }
                                    EnemyKind::Ancient => {
                                        menu_ancient_enemy_fight_victory(enemy.level, character)?;
                                    }
                                }
                                if character.data.stats.general_stats.character_level
                                    > character_level
                                {
                                    menu_level_up(
                                        character.data.stats.general_stats.character_level,
                                    )?;
                                }
                                character.temp_stat_boosts = player_temp_stat_boosts;
                                return Ok(true);
                            }
                            if player_turn {
                                player_turn = false;
                                match enemy.kind {
                                    EnemyKind::Boss | EnemyKind::Ancient => {
                                        if is_chance_success(ENEMY_SKILL_CHANCE) {
                                            let (event, effect) = enemy.use_skill(character);
                                            fight_text = event.to_string();
                                            effect_text = effect;
                                        } else {
                                            let (event, effect) = enemy.attack_player(character);
                                            fight_text = event.to_string();
                                            effect_text = effect;
                                        }
                                    }
                                    _ => {
                                        let (event, effect) = enemy.attack_player(character);
                                        fight_text = event.to_string();
                                        effect_text = effect;
                                    }
                                }
                            } else {
                                if character.is_dead() {
                                    character.increase_deaths();
                                    menu_enemy_fight_player_died(character)?;
                                    character.temp_stat_boosts = player_temp_stat_boosts;
                                    return Ok(false);
                                }
                                action = false;
                                player_turn = true;
                                fight_text = DEFAULT_FIGHT_TEXT.to_string();
                                effect_text = "".to_string();
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
    character.temp_stat_boosts = player_temp_stat_boosts;

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

fn menu_boss_enemy_fight_victory(
    enemy_level: u32,
    character: &mut PlayerCharacter,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    let drops = give_boss_enemy_drops(character, enemy_level);

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
        println!("  Items:");
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!(
            "    {} x{}",
            drops.consumable_item_name, drops.consumable_item_amount
        );
        execute!(stdout, cursor::MoveTo(0, 6))?;

        let mut column = 6;
        for item in &drops.equipment_item_names {
            println!("    {}", item);
            column += 1;
            execute!(stdout, cursor::MoveTo(0, column))?;
        }
        if drops.ancient_ruins_key {
            column += 1;
            println!("    Ancient Ruins Key x1");
        }
        execute!(stdout, cursor::MoveTo(0, column + 1))?;
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

fn menu_ancient_enemy_fight_victory(
    enemy_level: u32,
    character: &mut PlayerCharacter,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    let drops = give_ancient_enemy_drops(character, enemy_level);

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
        println!("  Items:");
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!(
            "    {} x{}",
            drops.consumable_item_name, drops.consumable_item_amount
        );
        execute!(stdout, cursor::MoveTo(0, 6))?;
        println!("    {}", drops.mythical_equipment_item_name);
        execute!(stdout, cursor::MoveTo(0, 8))?;
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
            "Highest Level Reached: {}",
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
            "Highest Dungeon Floor Reached: {}",
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

fn menu_enemy_fight_character_stats(character: &PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("(Esc) = Back");
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
