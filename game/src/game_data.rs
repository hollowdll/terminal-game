use base64::Engine;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::items::{ArmorItem, DisposableItem, RingItem, WeaponItem};
use std::{collections::HashMap, io};

const SAVEFILE_NAME: &str = "terminal_rpg_game_data";
const SUBDIR_NAME: &str = "terminal-rpg-game";

/// Main game data.
#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub game_characters: Vec<CharacterData>,
    pub achievements: Achievements,
}

impl GameData {
    pub fn serialize_to_json(&self) -> io::Result<String> {
        let json_str = serde_json::to_string(&self)?;
        Ok(json_str)
    }

    /// Creates new game data.
    pub fn new() -> GameData {
        GameData {
            game_characters: Vec::new(),
            achievements: Achievements {
                alltime_highest_dungeon_floor_record: 0,
                alltime_highest_character_level: 0,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Achievements {
    pub alltime_highest_dungeon_floor_record: u32,
    pub alltime_highest_character_level: u32,
}

/// Data of a game character.
#[derive(Serialize, Deserialize)]
pub struct CharacterData {
    pub metadata: CharacterMetadata,
    pub stats: CharacterStats,
    pub currency: CharacterCurrency,
    pub inventory: CharacterInventory,
    pub equipment: CharacterEquipment,
}

impl CharacterData {
    pub fn new(character_name: &str) -> Self {
        Self {
            metadata: CharacterMetadata {
                name: character_name.to_owned(),
                created_at: Utc::now().timestamp(),
            },
            stats: CharacterStats {
                general_stats: GeneralStats {
                    character_level: 1,
                    total_exp: 0,
                    current_exp: 0,
                    required_exp: 100,
                    current_dungeon_floor: 1,
                    highest_dungeon_floor_achieved: 1,
                    highest_character_level_achieved: 1,
                    deaths: 0,
                },
                combat_stats: CombatStats {
                    max_health: 100,
                    max_mana: 100,
                    defense: 0,
                    damage: 1,
                    critical_damage_multiplier: 2.0,
                    critical_hit_rate: 0.0,
                },
            },
            currency: CharacterCurrency { gold: 0 },
            inventory: CharacterInventory {
                disposable_items: HashMap::new(),
                armors: HashMap::new(),
                weapons: HashMap::new(),
                rings: HashMap::new(),
            },
            equipment: CharacterEquipment {
                weapon: None,
                armor: None,
                ring: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CharacterMetadata {
    /// Name of the character.
    pub name: String,
    /// Unix timestamp when the character was created in seconds.
    pub created_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterStats {
    pub general_stats: GeneralStats,
    pub combat_stats: CombatStats,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralStats {
    pub character_level: u32,
    pub total_exp: u32,
    pub current_exp: u32,
    pub required_exp: u32,
    pub current_dungeon_floor: u32,
    pub highest_dungeon_floor_achieved: u32,
    pub highest_character_level_achieved: u32,
    pub deaths: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CombatStats {
    pub max_health: u32,
    pub max_mana: u32,
    pub defense: u32,
    pub damage: u32,
    pub critical_damage_multiplier: f64,
    pub critical_hit_rate: f64,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterInventory {
    /// Hashmap key: item name.
    pub disposable_items: HashMap<String, DisposableItem>,
    /// HashMap key: item global id.
    pub armors: HashMap<String, ArmorItem>,
    /// HashMap key: item global id.
    pub weapons: HashMap<String, WeaponItem>,
    /// HashMap key: item global id.
    pub rings: HashMap<String, RingItem>,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterEquipment {
    pub weapon: Option<WeaponItem>,
    pub armor: Option<ArmorItem>,
    pub ring: Option<RingItem>,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterCurrency {
    pub gold: u64,
}

/// Creates the save file if it doesn't exist and overwrites it.
pub fn write_save_file(game_data: &GameData) -> io::Result<()> {
    let cache_subdir = get_cache_subdir(SUBDIR_NAME)?;
    let json_str = game_data.serialize_to_json()?;
    let encoded = base64::prelude::BASE64_STANDARD.encode(&json_str);

    let mut file = fs::File::create(cache_subdir.join(SAVEFILE_NAME))?;
    file.write_all(encoded.as_bytes())?;

    Ok(())
}

/// Reads the save file and loads the game data.
pub fn load_save_file() -> io::Result<GameData> {
    let cache_subdir = get_cache_subdir(SUBDIR_NAME)?;
    let content = fs::read_to_string(cache_subdir.join(SAVEFILE_NAME))?;
    let decoded = match base64::prelude::BASE64_STANDARD.decode(content) {
        Ok(decoded) => decoded,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("cannot decode base64: {}", e),
            ))
        }
    };
    let json_str = match String::from_utf8(decoded) {
        Ok(json_str) => json_str,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid utf8: {}", e),
            ))
        }
    };
    let game_data = deserialize_game_data_from_json(&json_str)?;

    Ok(game_data)
}

/// Creates new save file if it doesn't exist.
pub fn create_savefile_if_not_exists() -> io::Result<()> {
    let cache_subdir = get_cache_subdir(SUBDIR_NAME)?;
    let exists = cache_subdir.join(SAVEFILE_NAME).try_exists()?;
    if !exists {
        let game_data = GameData::new();
        write_save_file(&game_data)?;
    }

    Ok(())
}

/// Gets the path to the game's cache subdirectory.
/// Creates the directory if it doesn't exist.
pub fn get_cache_subdir(subdir: &str) -> io::Result<PathBuf> {
    let cache_dir = dirs::cache_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "cannot determine user's cache directory",
        )
    })?;
    let subdir_path = cache_dir.join(subdir);
    fs::create_dir_all(&subdir_path)?;

    Ok(subdir_path)
}

pub fn deserialize_game_data_from_json(json_str: &str) -> io::Result<GameData> {
    let game_data: GameData = serde_json::from_str(json_str)?;
    Ok(game_data)
}
