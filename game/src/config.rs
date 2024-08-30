const MAX_GAME_CHARACTERS: usize = 5;
const MAX_GAME_CHARACTER_NAME_LENGTH_BYTES: usize = 32;

pub struct GameConfig {
    pub max_game_characters: usize,
    pub max_game_character_name_length_bytes: usize,
}

impl GameConfig {
    pub fn new() -> Self {
        Self {
            max_game_characters: MAX_GAME_CHARACTERS,
            max_game_character_name_length_bytes: MAX_GAME_CHARACTER_NAME_LENGTH_BYTES,
        }
    }
}
