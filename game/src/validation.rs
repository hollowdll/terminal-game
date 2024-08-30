use crate::{config::GameConfig, session::Player};

pub fn character_name_too_long(cfg: &GameConfig, name: &str) -> bool {
    name.len() > cfg.max_game_character_name_length_bytes
}

pub fn character_name_already_exists(player: &Player, name: &str) -> bool {
    player.data.characters.contains_key(name)
}
