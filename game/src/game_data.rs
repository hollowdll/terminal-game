const MAX_GAME_CHARACTERS: u8 = 5;

/// Main game data.
pub struct GameData {
    pub game_characters: Vec<CharacterData>,
    pub achievements: Achievements,
}

pub struct Achievements {
    pub alltime_highest_dungeon_floor_record: u32,
    pub alltime_highest_character_level: u32,
}

/// Data of a game character.
pub struct CharacterData {
    pub metadata: CharacterMetadata,
    pub stats: CharacterStats,
    pub currency: CharacterCurrency,
    pub inventory: CharacterInventory,
    pub equipment: CharacterEquipment,
}

pub struct CharacterMetadata {
    /// Name of the character.
    pub name: String,
    /// Unix timestamp when the character was created in seconds.
    pub created_at: u64,
    /// Current time played since last death in seconds.
    pub current_time_played_seconds: u64,
}

pub struct CharacterStats {
    pub general_stats: GeneralStats,
    pub combat_stats: CombatStats,
    pub temp_stats: TemporaryStats,
    pub temp_stat_boosts: TemporaryStatBoosts,
}

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

pub struct CharacterInventory {}

pub struct CharacterEquipment {}

pub struct DisposableItem {
    pub name: String,
    pub description: String,
}

pub struct ArmorItem {
    pub name: String,
    pub base_health: u32,
    pub base_defense: u32,
}

pub struct WeaponItem {
    pub name: String,
    pub base_damage: u32,
}

pub struct CharacterCurrency {
    pub gold: u64,
}

pub struct Spell {
    pub name: String,
    pub description: String,
    pub mana_cost: u32,
}
