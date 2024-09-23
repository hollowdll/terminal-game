use core::fmt;

use crate::{
    config::GameConfig,
    game_data::{CharacterData, CombatStats},
    items::create_starter_weapon,
    session::{Player, PlayerCharacter},
};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub const BASE_EXP_MIN: u32 = 100;
pub const BASE_EXP_MAX: u32 = 125;

pub const SKILL_MANA_COST: u32 = 60;
pub const SKILL_DESCRIPTION_MAGIC_PROJECTILE: &str =
    "Deals 30% of the enemy's maximum health as pure damage.";
pub const SKILL_DESCRIPTION_RECOVER: &str = "Restores 45% of the player's maximum health points.";
pub const SKILL_DESCRIPTION_STEALTH: &str =
    "Increases the player's critical damage multiplier by 0.4 for the rest of the fight.";
pub const SKILL_DESCRIPTION_BATTLE_CRY: &str =
    "Increases the player's damage by 30% for the rest of the fight.";
pub const SKILL_DESCRIPTION_ARMOR_UP: &str =
    "Increases the player's defense equal to the player's level for the rest of the fight.";

pub const CLASS_MAGE_STARTING_STATS: CombatStats = CombatStats {
    max_health: 80,
    max_mana: 110,
    defense: 0,
    damage: 8,
    critical_damage_multiplier: 2.0,
    critical_hit_rate: 0.0,
};

pub const CLASS_CLERIC_STARTING_STATS: CombatStats = CombatStats {
    max_health: 100,
    max_mana: 100,
    defense: 0,
    damage: 5,
    critical_damage_multiplier: 2.0,
    critical_hit_rate: 0.0,
};

pub const CLASS_ASSASSIN_STARTING_STATS: CombatStats = CombatStats {
    max_health: 90,
    max_mana: 100,
    defense: 0,
    damage: 7,
    critical_damage_multiplier: 2.0,
    critical_hit_rate: 0.0,
};

pub const CLASS_WARRIOR_STARTING_STATS: CombatStats = CombatStats {
    max_health: 110,
    max_mana: 80,
    defense: 0,
    damage: 8,
    critical_damage_multiplier: 2.0,
    critical_hit_rate: 0.0,
};

pub const CLASS_KNIGHT_STARTING_STATS: CombatStats = CombatStats {
    max_health: 120,
    max_mana: 70,
    defense: 2,
    damage: 5,
    critical_damage_multiplier: 2.0,
    critical_hit_rate: 0.0,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterClass {
    Mage,
    Cleric,
    Assassin,
    Warrior,
    Knight,
}

#[derive(Debug)]
pub enum CharacterSkill {
    /// Mage
    MagicProjectile,
    /// Cleric
    Recover,
    /// Assassin
    Stealth,
    /// Warrior
    BattleCry,
    /// Knight
    ArmorUp,
}

impl fmt::Display for CharacterSkill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MagicProjectile => "Magic Projectile",
                Self::Recover => "Recover",
                Self::Stealth => "Stealth",
                Self::BattleCry => "Battle Cry",
                Self::ArmorUp => "Armor Up",
            }
        )
    }
}

pub fn max_game_characters_reached(player: &Player, cfg: &GameConfig) -> bool {
    return player.data.characters.len() >= cfg.max_game_characters;
}

pub fn get_character_skill(class: &CharacterClass) -> CharacterSkill {
    match class {
        CharacterClass::Mage => CharacterSkill::MagicProjectile,
        CharacterClass::Cleric => CharacterSkill::Recover,
        CharacterClass::Assassin => CharacterSkill::Stealth,
        CharacterClass::Warrior => CharacterSkill::BattleCry,
        CharacterClass::Knight => CharacterSkill::ArmorUp,
    }
}

pub fn get_character_skill_description(skill: &CharacterSkill) -> &str {
    match skill {
        CharacterSkill::MagicProjectile => SKILL_DESCRIPTION_MAGIC_PROJECTILE,
        CharacterSkill::Recover => SKILL_DESCRIPTION_RECOVER,
        CharacterSkill::Stealth => SKILL_DESCRIPTION_STEALTH,
        CharacterSkill::BattleCry => SKILL_DESCRIPTION_BATTLE_CRY,
        CharacterSkill::ArmorUp => SKILL_DESCRIPTION_ARMOR_UP,
    }
}

pub fn get_class_starting_stats(class: &CharacterClass) -> CombatStats {
    match class {
        CharacterClass::Mage => CLASS_MAGE_STARTING_STATS,
        CharacterClass::Cleric => CLASS_CLERIC_STARTING_STATS,
        CharacterClass::Assassin => CLASS_ASSASSIN_STARTING_STATS,
        CharacterClass::Warrior => CLASS_WARRIOR_STARTING_STATS,
        CharacterClass::Knight => CLASS_KNIGHT_STARTING_STATS,
    }
}

pub fn create_new_game_character(
    name: &str,
    class: CharacterClass,
    player: &mut Player,
    cfg: &GameConfig,
) {
    let character = CharacterData::new(name, class);
    let mut player_character = PlayerCharacter::new(&character);
    let weapon = create_starter_weapon(&player_character.data.metadata.class);
    player_character.give_weapon(&weapon);
    player_character.equip_weapon(&weapon.id);

    if cfg.dev_mode {
        player_character.give_test_items();
        player_character.give_gold(1500);
    }

    player.character = Some(player_character);
}

pub fn load_game_character(name: &str, player: &mut Player) {
    if let Some(character_data) = player.data.characters.get(name) {
        let mut character = PlayerCharacter::new(character_data);
        if let Some(weapon_id) = &character_data.equipment.weapon {
            character.equip_weapon(weapon_id);
        }
        if let Some(armor_id) = &character_data.equipment.armor {
            character.equip_armor(armor_id);
        }
        if let Some(ring_id) = &character_data.equipment.ring {
            character.equip_ring(ring_id);
        }
        player.character = Some(character);
    }
}

pub fn delete_game_character(name: &str, player: &mut Player) -> bool {
    if let Some(_) = player.data.characters.remove(name) {
        if let Some(plr_char) = &player.character {
            if plr_char.data.metadata.name.as_str() == name {
                player.character = None;
            }
        }
        return true;
    } else {
        return false;
    }
}

pub fn random_exp_amount(min_exp: u32, max_exp: u32, multiplier: u32) -> u32 {
    let mut rng = thread_rng();
    let base_exp = rng.gen_range(min_exp..=max_exp);
    return base_exp * multiplier;
}
