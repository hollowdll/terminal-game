const MAX_GAME_CHARACTERS: usize = 5;

pub struct GameConfig {
    pub max_game_characters: usize,
}

impl GameConfig {
    pub fn new() -> Self {
        Self {
            max_game_characters: MAX_GAME_CHARACTERS,
        }
    }
}
