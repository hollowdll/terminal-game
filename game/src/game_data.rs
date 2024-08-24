const MAX_GAME_CHARACTERS: u8 = 5;

/// Main game data.
pub struct GameData {
    pub game_characters: Vec<CharacterData>,
}

/// Data of a game character.
pub struct CharacterData {
    pub metadata: CharacterMetadata,
    pub stats: CharacterStats,
    pub currency: CharacterCurrency,
}

pub struct CharacterMetadata {
    /// Name of the character.
    pub name: String,
    /// Unix timestamp when the character was created in seconds.
    pub created_at: u64,
}

pub struct CharacterStats {
    pub health: u32,
    pub max_health: u32,
    pub mana: u32,
    pub max_mana: u32,
    pub armor: u32,
    pub damage: u32,
    pub critical_damage_multiplier: f64,
    pub critical_hit_rate: f64,
}

pub struct Inventory {}

pub struct CharacterEquipment {}

pub struct DisposableItem {}

pub struct EquipmentItem {}

pub struct CharacterCurrency {
    pub gold: u64,
}

pub struct Spell {
    pub name: String,
    pub desc: String,
    pub mana_cost: u32,
}
