use base64::Engine;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::{
    character::{get_class_starting_stats, CharacterClass},
    items::{ArmorItem, ConsumableItem, RingItem, WeaponItem},
};
use std::{collections::HashMap, io};

const SAVEFILE_NAME: &str = "terminal_rpg_game_data";
const SUBDIR_NAME: &str = "terminal-rpg-game";

pub const STARTING_REQUIRED_EXP: u32 = 100;
pub const STARTING_HEALTH: u32 = 100;
pub const STARTING_MANA: u32 = 100;
pub const STARTING_DEFENSE: u32 = 0;
pub const STARTING_DAMAGE: u32 = 1;
pub const STARTING_CRITICAL_DAMAGE_MULTIPLIER: f64 = 2.0;
pub const STARTING_CRITICAL_HIT_RATE: f64 = 0.0;

/// Main game data.
#[derive(Serialize, Deserialize)]
pub struct GameData {
    /// HashMap key: character name.
    pub characters: HashMap<String, CharacterData>,
}

impl GameData {
    pub fn serialize_to_json(&self) -> io::Result<String> {
        let json_str = serde_json::to_string(&self)?;
        Ok(json_str)
    }

    /// Creates new game data.
    pub fn new() -> GameData {
        GameData {
            characters: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Achievements {
    pub alltime_highest_dungeon_floor_record: u32,
    pub alltime_highest_character_level: u32,
}

/// Data of a game character.
#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterData {
    pub metadata: CharacterMetadata,
    pub stats: CharacterStats,
    pub currency: CharacterCurrency,
    pub inventory: CharacterInventory,
    pub equipment: CharacterEquipment,
}

impl CharacterData {
    pub fn new(character_name: &str, class: CharacterClass) -> Self {
        let starting_stats = get_class_starting_stats(&class);
        Self {
            metadata: CharacterMetadata {
                name: character_name.to_owned(),
                created_at: Utc::now().timestamp(),
                class,
            },
            stats: CharacterStats {
                general_stats: GeneralStats {
                    character_level: 1,
                    total_exp: 0,
                    current_exp: 0,
                    required_exp: STARTING_REQUIRED_EXP,
                    current_dungeon_floor: 1,
                    highest_dungeon_floor_achieved: 1,
                    highest_character_level_achieved: 1,
                    deaths: 0,
                },
                combat_stats: starting_stats,
            },
            currency: CharacterCurrency { gold: 0 },
            inventory: CharacterInventory {
                consumables: HashMap::new(),
                armors: HashMap::new(),
                weapons: HashMap::new(),
                rings: HashMap::new(),
                ancient_ruins_keys: 0,
            },
            equipment: CharacterEquipment {
                weapon: None,
                armor: None,
                ring: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterMetadata {
    /// Name of the character.
    pub name: String,
    /// Unix timestamp when the character was created in seconds.
    pub created_at: i64,
    pub class: CharacterClass,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterStats {
    pub general_stats: GeneralStats,
    pub combat_stats: CombatStats,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct CombatStats {
    pub max_health: u32,
    pub max_mana: u32,
    pub defense: u32,
    pub damage: u32,
    pub critical_damage_multiplier: f64,
    pub critical_hit_rate: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterInventory {
    /// Hashmap key: item name.
    pub consumables: HashMap<String, ConsumableItem>,
    /// HashMap key: item global id.
    pub armors: HashMap<String, ArmorItem>,
    /// HashMap key: item global id.
    pub weapons: HashMap<String, WeaponItem>,
    /// HashMap key: item global id.
    pub rings: HashMap<String, RingItem>,
    pub ancient_ruins_keys: u32,
}

impl CharacterInventory {
    pub fn clear_consumables(&mut self) {
        self.consumables.clear();
    }

    pub fn clear_weapons(&mut self) {
        self.weapons.clear();
    }

    pub fn clear_armors(&mut self) {
        self.armors.clear();
    }

    pub fn clear_rings(&mut self) {
        self.rings.clear();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterEquipment {
    /// ID of the item.
    pub weapon: Option<String>,
    /// ID of the item.
    pub armor: Option<String>,
    /// ID of the item.
    pub ring: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CharacterCurrency {
    pub gold: u32,
}

/// Creates the save file if it doesn't exist and overwrites it.
pub fn write_save_file(game_data: &GameData) -> io::Result<()> {
    let subdir = get_config_subdir(SUBDIR_NAME)?;
    let json_str = game_data.serialize_to_json()?;
    let encoded = base64::prelude::BASE64_STANDARD.encode(&json_str);

    let mut file = fs::File::create(subdir.join(SAVEFILE_NAME))?;
    file.write_all(encoded.as_bytes())?;

    Ok(())
}

/// Reads the save file and loads the game data.
pub fn load_save_file() -> io::Result<GameData> {
    let subdir = get_config_subdir(SUBDIR_NAME)?;
    let content = fs::read_to_string(subdir.join(SAVEFILE_NAME))?;
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
    let subdir = get_config_subdir(SUBDIR_NAME)?;
    let exists = subdir.join(SAVEFILE_NAME).try_exists()?;
    if !exists {
        let game_data = GameData::new();
        write_save_file(&game_data)?;
    }

    Ok(())
}

/// Gets the path to the game's cache subdirectory.
/// Creates the directory if it doesn't exist.
pub fn get_config_subdir(subdir: &str) -> io::Result<PathBuf> {
    let dir = dirs::config_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "cannot determine user's config directory",
        )
    })?;
    let subdir_path = dir.join(subdir);
    fs::create_dir_all(&subdir_path)?;

    Ok(subdir_path)
}

pub fn deserialize_game_data_from_json(json_str: &str) -> io::Result<GameData> {
    let game_data: GameData = serde_json::from_str(json_str)?;
    Ok(game_data)
}
