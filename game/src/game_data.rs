use serde::{Deserialize, Serialize};

use crate::items::{ArmorItem, DisposableItem, RingItem, WeaponItem};
use std::{collections::HashMap, io};

const MAX_GAME_CHARACTERS: u8 = 5;
const SAVEFILE_NAME: &str = "terminal_rpg_game_data";
const SUBDIR_NAME: &str = "terminal-rpg-game";

/// Main game data.
#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub game_characters: Vec<CharacterData>,
    pub achievements: Achievements,
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

#[derive(Serialize, Deserialize)]
pub struct CharacterMetadata {
    /// Name of the character.
    pub name: String,
    /// Unix timestamp when the character was created in seconds.
    pub created_at: u64,
    /// Current time played since last death in seconds.
    pub current_time_played_seconds: u64,
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

/// Don't save in savefile.
pub struct TemporaryStats {
    pub health: u32,
    pub mana: u32,
}

/// Don't save in savefile.
pub struct TemporaryStatBoosts {
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

pub fn write_save_file() -> io::Result<()> {
    Ok(())
}

pub fn load_save_file() -> io::Result<()> {
    Ok(())
}

pub fn create_save_file() -> io::Result<()> {
    Ok(())
}

pub fn serialize_game_data() -> io::Result<()> {
    Ok(())
}

pub fn deserialize_game_data() -> io::Result<()> {
    Ok(())
}
