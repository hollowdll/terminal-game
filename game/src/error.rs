use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    LoadSaveFile,
    NoSelectedCharacter,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LoadSaveFile => "Failed to load save file, it may be corrupted",
                Self::NoSelectedCharacter => "No selected character",
            }
        )
    }
}

impl Error for GameError {}
