use crate::{
    character::{get_character_skill, get_character_skill_description, SKILL_MANA_COST},
    session::PlayerCharacter,
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use std::io;

pub fn menu_skill(character: &PlayerCharacter) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;

    loop {
        let skill = get_character_skill(&character.data.metadata.class);
        execute!(stdout, cursor::MoveTo(0, 0))?;
        println!("Esc = Back");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!("Skill");
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!("  Name: {}", &skill);
        execute!(stdout, cursor::MoveTo(0, 3))?;
        println!(
            "  Description: {}",
            &get_character_skill_description(&skill)
        );
        execute!(stdout, cursor::MoveTo(0, 4))?;
        println!("  Mana Cost: {}", SKILL_MANA_COST);

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
