use crate::{
    character::{random_exp_amount, BASE_EXP_MAX, BASE_EXP_MIN},
    currency::{random_gold_amount, BASE_GOLD_MAX, BASE_GOLD_MIN, GOLD_MULTIPLIER_TREASURE_CHEST},
    enemy::{
        EXP_MULTIPLIER_BOSS_ENEMY, EXP_MULTIPLIER_NORMAL_ENEMY, GOLD_MULTIPLIER_BOSS_ENEMY,
        GOLD_MULTIPLIER_NORMAL_ENEMY,
    },
    items::{
        generate_random_armor, generate_random_consumable, generate_random_ring,
        generate_random_weapon, get_item_display_name, random_equipment_item, CharacterItem,
        ItemCategory, ARMOR_BASE_VALUES, RING_BASE_VALUES, WEAPON_BASE_VALUES,
    },
    session::PlayerCharacter,
    util::is_chance_success,
};

pub const ANCIENT_RUINS_KEY_DROP_RATE: f64 = 0.35;

pub struct NormalEnemyDrops {
    pub gold: u32,
    pub exp: u32,
    pub equipment_item_name: String,
}

pub struct BossEnemyDrops {
    pub gold: u32,
    pub exp: u32,
    pub equipment_item_names: Vec<String>,
    pub consumable_item_name: String,
    pub consumable_item_amount: u32,
    pub ancient_ruins_key: bool,
}

pub struct TreasureChestDrops {
    pub gold: u32,
    pub equipment_item_name: String,
}

pub fn give_normal_enemy_drops(
    character: &mut PlayerCharacter,
    enemy_level: u32,
) -> NormalEnemyDrops {
    let gold = random_gold_amount(BASE_GOLD_MIN, BASE_GOLD_MAX, GOLD_MULTIPLIER_NORMAL_ENEMY);
    character.give_gold(gold);
    let exp = random_exp_amount(BASE_EXP_MIN, BASE_EXP_MAX, EXP_MULTIPLIER_NORMAL_ENEMY);
    character.gain_exp(exp);

    let equipment_item_category = random_equipment_item();
    let mut item_display_name = "?Unknown?".to_string();
    match equipment_item_category {
        ItemCategory::Weapon => {
            let weapon = generate_random_weapon(
                WEAPON_BASE_VALUES,
                enemy_level,
                &character.data.metadata.class,
            );
            character.give_weapon(&weapon);
            item_display_name = get_item_display_name(CharacterItem::Weapon(&weapon));
        }
        ItemCategory::Armor => {
            let armor = generate_random_armor(ARMOR_BASE_VALUES, enemy_level);
            character.give_armor(&armor);
            item_display_name = get_item_display_name(CharacterItem::Armor(&armor));
        }
        ItemCategory::Ring => {
            let ring = generate_random_ring(RING_BASE_VALUES, enemy_level);
            character.give_ring(&ring);
            item_display_name = get_item_display_name(CharacterItem::Ring(&ring));
        }
        _ => {}
    }

    NormalEnemyDrops {
        gold,
        exp,
        equipment_item_name: item_display_name,
    }
}

pub fn give_boss_enemy_drops(character: &mut PlayerCharacter, enemy_level: u32) -> BossEnemyDrops {
    let gold = random_gold_amount(BASE_GOLD_MIN, BASE_GOLD_MAX, GOLD_MULTIPLIER_BOSS_ENEMY);
    character.give_gold(gold);
    let exp = random_exp_amount(BASE_EXP_MIN, BASE_EXP_MAX, EXP_MULTIPLIER_BOSS_ENEMY);
    character.gain_exp(exp);

    let mut item_display_names = Vec::new();
    for _ in 0..2 {
        let equipment_item_category = random_equipment_item();
        match equipment_item_category {
            ItemCategory::Weapon => {
                let weapon = generate_random_weapon(
                    WEAPON_BASE_VALUES,
                    enemy_level,
                    &character.data.metadata.class,
                );
                character.give_weapon(&weapon);
                item_display_names.push(get_item_display_name(CharacterItem::Weapon(&weapon)));
            }
            ItemCategory::Armor => {
                let armor = generate_random_armor(ARMOR_BASE_VALUES, enemy_level);
                character.give_armor(&armor);
                item_display_names.push(get_item_display_name(CharacterItem::Armor(&armor)));
            }
            ItemCategory::Ring => {
                let ring = generate_random_ring(RING_BASE_VALUES, enemy_level);
                character.give_ring(&ring);
                item_display_names.push(get_item_display_name(CharacterItem::Ring(&ring)));
            }
            _ => {}
        }
    }
    let consumable = generate_random_consumable();
    character.give_consumable(&consumable, 1);
    let ancient_ruins_key = is_chance_success(ANCIENT_RUINS_KEY_DROP_RATE);
    if ancient_ruins_key {
        character.give_ancient_ruins_key(1);
    }

    BossEnemyDrops {
        gold,
        exp,
        equipment_item_names: item_display_names,
        consumable_item_name: get_item_display_name(CharacterItem::Consumable(&consumable)),
        consumable_item_amount: 1,
        ancient_ruins_key,
    }
}

pub fn give_treasure_chest_drops(
    character: &mut PlayerCharacter,
    dungeon_floor: u32,
) -> TreasureChestDrops {
    let gold = random_gold_amount(BASE_GOLD_MIN, BASE_GOLD_MAX, GOLD_MULTIPLIER_TREASURE_CHEST);
    character.give_gold(gold);

    let equipment_item_category = random_equipment_item();
    let mut item_display_name = "?Unknown?".to_string();
    match equipment_item_category {
        ItemCategory::Weapon => {
            let weapon = generate_random_weapon(
                WEAPON_BASE_VALUES,
                dungeon_floor,
                &character.data.metadata.class,
            );
            character.give_weapon(&weapon);
            item_display_name = get_item_display_name(CharacterItem::Weapon(&weapon));
        }
        ItemCategory::Armor => {
            let armor = generate_random_armor(ARMOR_BASE_VALUES, dungeon_floor);
            character.give_armor(&armor);
            item_display_name = get_item_display_name(CharacterItem::Armor(&armor));
        }
        ItemCategory::Ring => {
            let ring = generate_random_ring(RING_BASE_VALUES, dungeon_floor);
            character.give_ring(&ring);
            item_display_name = get_item_display_name(CharacterItem::Ring(&ring));
        }
        _ => {}
    }

    TreasureChestDrops {
        gold,
        equipment_item_name: item_display_name,
    }
}
