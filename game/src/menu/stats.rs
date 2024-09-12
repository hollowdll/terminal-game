use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{session::PlayerCharacter, util::timestamp_to_datetime};

pub fn menu_character_stats(character: &PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let menu_items = vec!["Back"];
    let mut selected_index = 0;
    let start_column: u16 = 22;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Character Stats");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("  General:");
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("    Name: {}", character.data.metadata.name);
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!(
            "    Created At: {}",
            timestamp_to_datetime(character.data.metadata.created_at)
        );
        execute!(stdout, cursor::MoveTo(0, 4))?;
        println!(
            "    Current Dungeon Floor: {}",
            character.data.stats.general_stats.current_dungeon_floor
        );
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!(
            "    Level: {}",
            character.data.stats.general_stats.character_level
        );
        execute!(stdout, cursor::MoveTo(0, 6))?;
        println!(
            "    EXP: {}",
            character.data.stats.general_stats.current_exp
        );
        execute!(stdout, cursor::MoveTo(0, 7))?;
        println!(
            "    Required EXP: {}",
            character.data.stats.general_stats.required_exp
        );
        execute!(stdout, cursor::MoveTo(0, 8))?;
        println!(
            "    Total EXP: {}",
            character.data.stats.general_stats.total_exp
        );
        execute!(stdout, cursor::MoveTo(0, 9))?;

        println!(
            "    Highest Dungeon Floor Reached: {}",
            character
                .data
                .stats
                .general_stats
                .highest_dungeon_floor_achieved
        );
        execute!(stdout, cursor::MoveTo(0, 10))?;
        println!(
            "    Highest Level Reached: {}",
            character
                .data
                .stats
                .general_stats
                .highest_character_level_achieved
        );
        execute!(stdout, cursor::MoveTo(0, 11))?;
        println!(
            "    Total Deaths: {}",
            character.data.stats.general_stats.deaths
        );
        execute!(stdout, cursor::MoveTo(0, 12))?;

        println!("  Combat:");
        execute!(stdout, cursor::MoveTo(0, 13))?;
        println!(
            "    Max Health: {}",
            character.data.stats.combat_stats.max_health
        );
        execute!(stdout, cursor::MoveTo(0, 14))?;
        println!("    Health: {}", character.temp_stats.health);
        execute!(stdout, cursor::MoveTo(0, 15))?;
        println!(
            "    Max Mana: {}",
            character.data.stats.combat_stats.max_mana
        );
        execute!(stdout, cursor::MoveTo(0, 16))?;
        println!("    Mana: {}", character.temp_stats.mana);
        execute!(stdout, cursor::MoveTo(0, 17))?;
        println!("    Defense: {}", character.data.stats.combat_stats.defense);
        execute!(stdout, cursor::MoveTo(0, 18))?;
        println!("    Damage: {}", character.data.stats.combat_stats.damage);
        execute!(stdout, cursor::MoveTo(0, 19))?;
        println!(
            "    Critical Damage Multiplier: {}",
            character.data.stats.combat_stats.critical_damage_multiplier
        );
        execute!(stdout, cursor::MoveTo(0, 20))?;
        println!(
            "    Critical Hit Rate: {}",
            character.data.stats.combat_stats.critical_hit_rate
        );
        execute!(stdout, cursor::MoveTo(0, 21))?;

        println!("");
        execute!(stdout, cursor::MoveTo(0, 22))?;

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
    execute!(stdout, Clear(ClearType::All))?;

    Ok(())
}
