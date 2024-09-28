use crate::{game_data::write_save_file, session::Player};
use std::io;

pub enum StatusBar {
    Health,
    Mana,
}

pub fn save_game(player: &mut Player) -> io::Result<()> {
    if let Some(player_character) = &mut player.character {
        player_character.data.equipment.weapon = player_character.equipped_items.weapon.clone();
        player_character.data.equipment.armor = player_character.equipped_items.armor.clone();
        player_character.data.equipment.ring = player_character.equipped_items.ring.clone();

        player.data.characters.insert(
            player_character.data.metadata.name.clone(),
            player_character.data.clone(),
        );
    }
    write_save_file(&player.data)?;
    Ok(())
}
