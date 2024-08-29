use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    LoadSaveFile,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LoadSaveFile => "Failed to load save file, it may be corrupted",
            }
        )
    }
}

impl Error for GameError {}
