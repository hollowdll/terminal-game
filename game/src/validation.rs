use crate::session::Player;

pub const MAX_GAME_CHARACTERS: usize = 5;
pub const MAX_GAME_CHARACTER_NAME_LENGTH_BYTES: usize = 32;

pub fn character_name_too_long(name: &str) -> bool {
    name.len() > MAX_GAME_CHARACTER_NAME_LENGTH_BYTES
}

pub fn character_name_empty(name: &str) -> bool {
    name.trim().is_empty()
}

pub fn character_name_already_exists(player: &Player, name: &str) -> bool {
    player.data.characters.contains_key(name)
}
