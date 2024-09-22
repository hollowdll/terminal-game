use crate::{
    config::GameConfig,
    game_data::CharacterData,
    items::create_starter_weapon,
    session::{Player, PlayerCharacter},
};
use rand::{thread_rng, Rng};

pub const BASE_EXP_MIN: u32 = 100;
pub const BASE_EXP_MAX: u32 = 125;

pub const SKILL_DESCRIPTION_MAGIC_PROJECTILE: &str =
    "Deals 20% of the enemy's maximum health as damage.";
pub const SKILL_DESCRIPTION_RECOVER: &str = "Restores 35% of the player's maximum health points.";
pub const SKILL_DESCRIPTION_STEALTH: &str =
    "Increases the player's critical damage multiplier by 0.3 for the rest of the fight.";
pub const SKILL_DESCRIPTION_BATTLE_CRY: &str =
    "Increases the player's damage by 30% for the rest of the fight.";
pub const SKILL_DESCRIPTION_ARMOR_UP: &str =
    "Increases the player's defense equal to the player's level for the rest of the fight.";

#[derive(Debug)]
pub enum CharacterClass {
    Mage,
    Cleric,
    Assassin,
    Warrior,
    Knight,
}

#[derive(Debug)]
pub enum CharacterSkill {
    MagicProjectile,
    Recover,
    Stealth,
    BattleCry,
    ArmorUp,
}

pub fn max_game_characters_reached(player: &Player, cfg: &GameConfig) -> bool {
    return player.data.characters.len() >= cfg.max_game_characters;
}

pub fn create_new_game_character(character_name: &str, player: &mut Player, cfg: &GameConfig) {
    let character = CharacterData::new(character_name);
    let mut player_character = PlayerCharacter::new(&character);
    let weapon = create_starter_weapon();
    player_character.give_weapon(&weapon);
    player_character.equip_weapon(&weapon.id);

    if cfg.dev_mode {
        player_character.give_test_items();
        player_character.give_gold(1500);
    }

    player.character = Some(player_character);
}

pub fn load_game_character(character_name: &str, player: &mut Player) {
    if let Some(character_data) = player.data.characters.get(character_name) {
        let mut character = PlayerCharacter::new(character_data);
        if let Some(weapon_id) = &character_data.equipment.weapon {
            character.equip_weapon(weapon_id);
        }
        player.character = Some(character);
    }
}

pub fn delete_game_character(character_name: &str, player: &mut Player) -> bool {
    if let Some(_) = player.data.characters.remove(character_name) {
        if let Some(plr_char) = &player.character {
            if plr_char.data.metadata.name.as_str() == character_name {
                player.character = None;
            }
        }
        return true;
    } else {
        return false;
    }
}

pub fn random_exp_amount(min_exp: u32, max_exp: u32, multiplier: u32, dungeon_floor: u32) -> u32 {
    let mut rng = thread_rng();
    let base_exp = rng.gen_range(min_exp..=max_exp);
    return base_exp * multiplier + (base_exp / 4 * dungeon_floor);
}
