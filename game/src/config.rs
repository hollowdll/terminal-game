use crate::util::is_dev_mode;

pub struct GameConfig {
    pub dev_mode: bool,
}

impl GameConfig {
    pub fn new() -> Self {
        Self {
            dev_mode: is_dev_mode(),
        }
    }
}
