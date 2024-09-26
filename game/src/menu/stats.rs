use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{session::PlayerCharacter, util::timestamp_to_datetime};

pub fn menu_character_stats(character: &PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Character Stats");
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("  General:");
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!("    Name: {}", character.data.metadata.name);
        execute!(stdout, cursor::MoveTo(0, 4))?;
        println!("    Class: {:?}", character.data.metadata.class);
        execute!(stdout, cursor::MoveTo(0, 5))?;
        println!(
            "    Created At: {}",
            timestamp_to_datetime(character.data.metadata.created_at)
        );
        execute!(stdout, cursor::MoveTo(0, 6))?;
        println!(
            "    Current Dungeon Floor: {}",
            character.data.stats.general_stats.current_dungeon_floor
        );
        execute!(stdout, cursor::MoveTo(0, 7))?;
        println!(
            "    Level: {}",
            character.data.stats.general_stats.character_level
        );
        execute!(stdout, cursor::MoveTo(0, 8))?;
        println!(
            "    EXP: {}/{}",
            character.data.stats.general_stats.current_exp,
            character.data.stats.general_stats.required_exp,
        );
        execute!(stdout, cursor::MoveTo(0, 9))?;
        println!(
            "    Total EXP: {}",
            character.data.stats.general_stats.total_exp
        );
        execute!(stdout, cursor::MoveTo(0, 10))?;

        println!(
            "    Highest Dungeon Floor Reached: {}",
            character
                .data
                .stats
                .general_stats
                .highest_dungeon_floor_achieved
        );
        execute!(stdout, cursor::MoveTo(0, 11))?;
        println!(
            "    Highest Level Reached: {}",
            character
                .data
                .stats
                .general_stats
                .highest_character_level_achieved
        );
        execute!(stdout, cursor::MoveTo(0, 12))?;
        println!(
            "    Total Deaths: {}",
            character.data.stats.general_stats.deaths
        );
        execute!(stdout, cursor::MoveTo(0, 13))?;

        println!("  Combat:");
        execute!(stdout, cursor::MoveTo(0, 14))?;
        println!("    Health: {}", character.get_total_health());
        execute!(stdout, cursor::MoveTo(0, 15))?;
        println!("    Mana: {}", character.get_total_mana());
        execute!(stdout, cursor::MoveTo(0, 16))?;
        println!("    Defense: {}", character.get_total_defense());
        execute!(stdout, cursor::MoveTo(0, 17))?;
        println!("    Damage: {}", character.get_total_damage());
        execute!(stdout, cursor::MoveTo(0, 18))?;
        println!(
            "    Critical Damage Multiplier: {:.2}",
            character.get_total_crit_damage_multiplier()
        );
        execute!(stdout, cursor::MoveTo(0, 19))?;
        println!(
            "    Critical Hit Rate: {:.2} ({:.2}%)",
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
