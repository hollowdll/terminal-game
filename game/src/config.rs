const MAX_GAME_CHARACTERS: u8 = 5;

pub struct GameConfig {
    pub max_game_characters: u8,
}

impl GameConfig {
    pub fn new() -> Self {
        Self {
            max_game_characters: MAX_GAME_CHARACTERS,
        }
    }
}
