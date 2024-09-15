use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

use crate::{enemy::Enemy, session::PlayerCharacter};

/// Returns true if the player wins the fight.
pub fn menu_enemy_encounter(
    enemy: &mut Enemy,
    _character: &mut PlayerCharacter,
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
    execute!(stdout, Clear(ClearType::All))?;

    Ok(true)
}
