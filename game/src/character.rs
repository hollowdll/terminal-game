use crate::{
    config::GameConfig,
    game_data::CharacterData,
    items::create_starter_weapon,
    session::{Player, PlayerCharacter},
};

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

pub fn delete_game_character(player: &mut Player, character_name: &str) -> bool {
    if let Some(_) = player.data.characters.remove(character_name) {
        if let Some(plr_char) = &player.character {
            if plr_char.data.metadata.name.as_str() == character_name {
                player.character = None;
            }
        }
        return true;
    } else {
        return false;
    }
}
