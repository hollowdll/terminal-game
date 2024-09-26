use crate::{
    character::{random_exp_amount, BASE_EXP_MAX, BASE_EXP_MIN},
    currency::{random_gold_amount, BASE_GOLD_MAX, BASE_GOLD_MIN, GOLD_MULTIPLIER_TREASURE_CHEST},
    enemy::{
        EXP_MULTIPLIER_ANCIENT_ENEMY, EXP_MULTIPLIER_BOSS_ENEMY, EXP_MULTIPLIER_NORMAL_ENEMY,
        GOLD_MULTIPLIER_ANCIENT_ENEMY, GOLD_MULTIPLIER_BOSS_ENEMY, GOLD_MULTIPLIER_NORMAL_ENEMY,
    },
    items::{
        generate_random_armor, generate_random_consumable, generate_random_ring,
        generate_random_weapon, get_item_display_name, random_equipment_item, random_item_rarity,
        CharacterItem, ItemCategory, ItemRarity, ARMOR_BASE_VALUES, ITEM_RARITY_DROP_RATES,
        RING_BASE_VALUES, WEAPON_BASE_VALUES,
    },
    session::PlayerCharacter,
    util::is_chance_success,
};

pub const ANCIENT_RUINS_KEY_DROP_RATE: f64 = 0.40;

pub struct ItemDrop {
    pub name: String,
    pub rarity: ItemRarity,
    pub lvl: u32,
}

pub struct NormalEnemyDrops {
    pub gold: u32,
    pub exp: u32,
    pub equipment_item: ItemDrop,
}

pub struct BossEnemyDrops {
    pub gold: u32,
    pub exp: u32,
    pub equipment_items: Vec<ItemDrop>,
    pub consumable_item: ItemDrop,
    pub consumable_item_amount: u32,
    pub ancient_ruins_key: bool,
}

pub struct AncientEnemyDrops {
    pub gold: u32,
    pub exp: u32,
    pub equipment_item: ItemDrop,
    pub consumable_item: ItemDrop,
    pub consumable_item_amount: u32,
}

pub struct TreasureChestDrops {
    pub gold: u32,
    pub equipment_item: ItemDrop,
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
    let equipment_item = match equipment_item_category {
        ItemCategory::Weapon => {
            let weapon = generate_random_weapon(
                random_item_rarity(&ITEM_RARITY_DROP_RATES),
                WEAPON_BASE_VALUES,
                enemy_level,
                &character.data.metadata.class,
            );
            character.give_weapon(&weapon);
            ItemDrop {
                name: get_item_display_name(CharacterItem::Weapon(&weapon)),
                rarity: weapon.rarity,
                lvl: weapon.level,
            }
        }
        ItemCategory::Armor => {
            let armor = generate_random_armor(
                random_item_rarity(&ITEM_RARITY_DROP_RATES),
                ARMOR_BASE_VALUES,
                enemy_level,
            );
            character.give_armor(&armor);
            ItemDrop {
                name: get_item_display_name(CharacterItem::Armor(&armor)),
                rarity: armor.rarity,
                lvl: armor.level,
            }
        }
        ItemCategory::Ring => {
            let ring = generate_random_ring(
                random_item_rarity(&ITEM_RARITY_DROP_RATES),
                RING_BASE_VALUES,
                enemy_level,
            );
            character.give_ring(&ring);
            ItemDrop {
                name: get_item_display_name(CharacterItem::Ring(&ring)),
                rarity: ring.rarity,
                lvl: ring.level,
            }
        }
        _ => ItemDrop {
            name: "?Unknown?".to_owned(),
            rarity: ItemRarity::Unknown,
            lvl: 0,
        },
    };

    NormalEnemyDrops {
        gold,
        exp,
        equipment_item,
    }
}

pub fn give_boss_enemy_drops(character: &mut PlayerCharacter, enemy_level: u32) -> BossEnemyDrops {
    let gold = random_gold_amount(BASE_GOLD_MIN, BASE_GOLD_MAX, GOLD_MULTIPLIER_BOSS_ENEMY);
    character.give_gold(gold);
    let exp = random_exp_amount(BASE_EXP_MIN, BASE_EXP_MAX, EXP_MULTIPLIER_BOSS_ENEMY);
    character.gain_exp(exp);

    let mut equipment_drops = Vec::new();
    for _ in 0..2 {
        let equipment_item_category = random_equipment_item();
        match equipment_item_category {
            ItemCategory::Weapon => {
                let weapon = generate_random_weapon(
                    random_item_rarity(&ITEM_RARITY_DROP_RATES),
                    WEAPON_BASE_VALUES,
                    enemy_level,
                    &character.data.metadata.class,
                );
                character.give_weapon(&weapon);
                equipment_drops.push(ItemDrop {
                    name: get_item_display_name(CharacterItem::Weapon(&weapon)),
                    rarity: weapon.rarity,
                    lvl: weapon.level,
                });
            }
            ItemCategory::Armor => {
                let armor = generate_random_armor(
                    random_item_rarity(&ITEM_RARITY_DROP_RATES),
                    ARMOR_BASE_VALUES,
                    enemy_level,
                );
                character.give_armor(&armor);
                equipment_drops.push(ItemDrop {
                    name: get_item_display_name(CharacterItem::Armor(&armor)),
                    rarity: armor.rarity,
                    lvl: armor.level,
                });
            }
            ItemCategory::Ring => {
                let ring = generate_random_ring(
                    random_item_rarity(&ITEM_RARITY_DROP_RATES),
                    RING_BASE_VALUES,
                    enemy_level,
                );
                character.give_ring(&ring);
                equipment_drops.push(ItemDrop {
                    name: get_item_display_name(CharacterItem::Ring(&ring)),
                    rarity: ring.rarity,
                    lvl: ring.level,
                });
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
        equipment_items: equipment_drops,
        consumable_item: ItemDrop {
            name: get_item_display_name(CharacterItem::Consumable(&consumable)),
            rarity: consumable.rarity,
            lvl: 1,
        },
        consumable_item_amount: 1,
        ancient_ruins_key,
    }
}

pub fn give_ancient_enemy_drops(
    character: &mut PlayerCharacter,
    enemy_level: u32,
) -> AncientEnemyDrops {
    let gold = random_gold_amount(BASE_GOLD_MIN, BASE_GOLD_MAX, GOLD_MULTIPLIER_ANCIENT_ENEMY);
    character.give_gold(gold);
    let exp = random_exp_amount(BASE_EXP_MIN, BASE_EXP_MAX, EXP_MULTIPLIER_ANCIENT_ENEMY);
    character.gain_exp(exp);

    let equipment_item_category = random_equipment_item();
    let equipment_item = match equipment_item_category {
        ItemCategory::Weapon => {
            let weapon = generate_random_weapon(
                ItemRarity::Mythical,
                WEAPON_BASE_VALUES,
                enemy_level,
                &character.data.metadata.class,
            );
            character.give_weapon(&weapon);
            ItemDrop {
                name: get_item_display_name(CharacterItem::Weapon(&weapon)),
                rarity: weapon.rarity,
                lvl: weapon.level,
            }
        }
        ItemCategory::Armor => {
            let armor = generate_random_armor(ItemRarity::Mythical, ARMOR_BASE_VALUES, enemy_level);
            character.give_armor(&armor);
            ItemDrop {
                name: get_item_display_name(CharacterItem::Armor(&armor)),
                rarity: armor.rarity,
                lvl: armor.level,
            }
        }
        ItemCategory::Ring => {
            let ring = generate_random_ring(ItemRarity::Mythical, RING_BASE_VALUES, enemy_level);
            character.give_ring(&ring);
            ItemDrop {
                name: get_item_display_name(CharacterItem::Ring(&ring)),
                rarity: ring.rarity,
                lvl: ring.level,
            }
        }
        _ => ItemDrop {
            name: "?Unknown?".to_owned(),
            rarity: ItemRarity::Unknown,
            lvl: 0,
        },
    };
    let consumable = generate_random_consumable();
    character.give_consumable(&consumable, 3);

    AncientEnemyDrops {
        gold,
        exp,
        equipment_item,
        consumable_item: ItemDrop {
            name: get_item_display_name(CharacterItem::Consumable(&consumable)),
            rarity: consumable.rarity,
            lvl: 1,
        },
        consumable_item_amount: 3,
    }
}

pub fn give_treasure_chest_drops(
    character: &mut PlayerCharacter,
    level: u32,
) -> TreasureChestDrops {
    let gold = random_gold_amount(BASE_GOLD_MIN, BASE_GOLD_MAX, GOLD_MULTIPLIER_TREASURE_CHEST);
    character.give_gold(gold);

    let equipment_item_category = random_equipment_item();
    let equipment_item = match equipment_item_category {
        ItemCategory::Weapon => {
            let weapon = generate_random_weapon(
                random_item_rarity(&ITEM_RARITY_DROP_RATES),
                WEAPON_BASE_VALUES,
                level,
                &character.data.metadata.class,
            );
            character.give_weapon(&weapon);
            ItemDrop {
                name: get_item_display_name(CharacterItem::Weapon(&weapon)),
                rarity: weapon.rarity,
                lvl: weapon.level,
            }
        }
        ItemCategory::Armor => {
            let armor = generate_random_armor(
                random_item_rarity(&ITEM_RARITY_DROP_RATES),
                ARMOR_BASE_VALUES,
                level,
            );
            character.give_armor(&armor);
            ItemDrop {
                name: get_item_display_name(CharacterItem::Armor(&armor)),
                rarity: armor.rarity,
                lvl: armor.level,
            }
        }
        ItemCategory::Ring => {
            let ring = generate_random_ring(
                random_item_rarity(&ITEM_RARITY_DROP_RATES),
                RING_BASE_VALUES,
                level,
            );
            character.give_ring(&ring);
            ItemDrop {
                name: get_item_display_name(CharacterItem::Ring(&ring)),
                rarity: ring.rarity,
                lvl: ring.level,
            }
        }
        _ => ItemDrop {
            name: "?Unknown?".to_owned(),
            rarity: ItemRarity::Unknown,
            lvl: 0,
        },
    };

    TreasureChestDrops {
        gold,
        equipment_item,
    }
}
