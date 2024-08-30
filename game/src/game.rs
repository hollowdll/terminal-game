use std::io;

use crate::{game_data::write_save_file, session::Player};

pub fn save_game(player: &mut Player) -> io::Result<()> {
    if let Some(player_character) = &player.character {
        player.data.characters.insert(
            player_character.data.metadata.name.clone(),
            player_character.data.clone(),
        );
    }
    write_save_file(&player.data)?;
    Ok(())
}
