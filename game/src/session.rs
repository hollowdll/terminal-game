use crate::{
    game_data::{CharacterData, GameData},
    items::WeaponItem,
};

pub struct Player {
    pub character: Option<PlayerCharacter>,
    pub data: GameData,
}

impl Player {
    pub fn new(data: GameData) -> Self {
        Self {
            character: None,
            data,
        }
    }
}

pub struct PlayerCharacter {
    pub data: CharacterData,
    pub temp_stats: TemporaryStats,
    pub temp_stat_boosts: TemporaryStatBoosts,
}

impl PlayerCharacter {
    pub fn new(data: &CharacterData) -> Self {
        Self {
            data: data.clone(),
            temp_stats: TemporaryStats {
                health: data.stats.combat_stats.max_health,
                mana: data.stats.combat_stats.max_mana,
            },
            temp_stat_boosts: TemporaryStatBoosts {
                max_health: 0,
                max_mana: 0,
                defense: 0,
                damage: 0,
                critical_damage_multiplier: 0.0,
                critical_hit_rate: 0.0,
            },
        }
    }

    pub fn give_gold(&mut self, amount: u64) {
        self.data.currency.gold += amount;
    }

    pub fn give_weapon(&mut self, weapon: &WeaponItem) {
        self.data
            .inventory
            .weapons
            .insert(weapon.global_id.clone(), weapon.clone());
    }

    /// The returned bool is true if the weapon is in the inventory and it was equipped.
    /// If the item doesn't exist in the inventory, the returned bool is false.
    pub fn equip_weapon(&mut self, weapon_id: &str) -> bool {
        match self.data.inventory.weapons.get(weapon_id) {
            Some(weapon) => {
                self.data.equipment.weapon = Some(weapon.clone());
                return true;
            }
            None => return false,
        }
    }
}

pub struct TemporaryStats {
    pub health: u32,
    pub mana: u32,
}

pub struct TemporaryStatBoosts {
    pub max_health: u32,
    pub max_mana: u32,
    pub defense: u32,
    pub damage: u32,
    pub critical_damage_multiplier: f64,
    pub critical_hit_rate: f64,
}
