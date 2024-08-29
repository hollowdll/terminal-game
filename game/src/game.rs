use std::io;

use crate::{
    config::GameConfig,
    game_data::{write_save_file, CharacterData},
    items::create_starter_weapon,
    session::{Player, PlayerCharacter},
};

// pub fn new_game(_player: &mut Player, _cfg: &GameConfig) {}

pub fn max_game_characters_reached(player: &Player, cfg: &GameConfig) -> bool {
    return player.data.characters.len() >= cfg.max_game_characters;
}

pub fn create_new_game_character(player: &mut Player, character_name: &str) {
    let character = CharacterData::new(character_name);
    let mut player_character = PlayerCharacter::new(&character);
    let weapon = create_starter_weapon();
    player_character.give_weapon(&weapon);
    player_character.equip_weapon(&weapon.global_id);
    player.character = Some(player_character);
}

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
