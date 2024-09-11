use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

pub const ITEM_RARITY_DROP_RATES: ItemRarityDropRates = ItemRarityDropRates {
    common: 0.35,
    uncommon: 0.30,
    rare: 0.20,
    epic: 0.10,
    legendary: 0.05,
};
pub const WEAPON_BASE_VALUES: WeaponBaseValues = WeaponBaseValues {
    min_damage: 12,
    max_damage: 15,
    min_crit_hit_rate: 0.15,
    max_crit_hit_rate: 0.20,
};
pub const ARMOR_BASE_VALUES: ArmorBaseValues = ArmorBaseValues {
    min_health: 20,
    max_health: 25,
    min_defense: 1,
    max_defense: 3,
};
pub const RING_BASE_VALUES: RingBaseValues = RingBaseValues {
    min_mana: 20,
    max_mana: 25,
};
pub const ENCHANTMENT_BASE_VALUES: EnchantmentBaseValues = EnchantmentBaseValues {
    min_damage: 5,
    max_damage: 7,
    min_crit_hit_rate: 0.08,
    max_crit_hit_rate: 0.12,
    min_health: 10,
    max_health: 15,
    min_defense: 1,
    max_defense: 2,
    min_mana: 10,
    max_mana: 15,
};

//-------------------//
// Disposable items //
//-----------------//

pub const ITEM_HEALTH_POTION: ItemInfo = ItemInfo {
    name: Cow::Borrowed("Health Potion"),
    description: Cow::Borrowed("A magical potion that restores health points."),
    category: ItemCategory::Consumable,
};

//---------------//
// Weapon items //
//-------------//

pub const ITEM_SWORD: ItemInfo = ItemInfo {
    name: Cow::Borrowed("Sword"),
    description: Cow::Borrowed("A sharp sword that can be used for fighting."),
    category: ItemCategory::Weapon,
};

//--------------//
// Armor items //
//------------//

pub const ITEM_ARMOR: ItemInfo = ItemInfo {
    name: Cow::Borrowed("Armor"),
    description: Cow::Borrowed("A defensive armor."),
    category: ItemCategory::Armor,
};

//--------//
// Rings //
//------//

pub const ITEM_RING: ItemInfo = ItemInfo {
    name: Cow::Borrowed("Ring"),
    description: Cow::Borrowed("A ring that increases some stats."),
    category: ItemCategory::Ring,
};

//-----------------------------------//

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemInfo {
    pub name: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub category: ItemCategory,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConsumableItem {
    pub info: ItemInfo,
    pub effect: String,
    pub rarity: ItemRarity,
    pub amount_in_inventory: u32,
}

impl ConsumableItem {
    pub fn new_health_potion(rarity: ItemRarity) -> Self {
        Self {
            info: ITEM_HEALTH_POTION,
            effect: get_health_potion_effect(&rarity),
            rarity,
            amount_in_inventory: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArmorItem {
    pub info: ItemInfo,
    pub id: String,
    pub level: u32,
    pub rarity: ItemRarity,
    pub health: u32,
    pub defense: u32,
    pub enchantments: Vec<Enchantment>,
}

impl ArmorItem {
    pub fn new(
        info: ItemInfo,
        level: u32,
        rarity: ItemRarity,
        health: u32,
        defense: u32,
        enchantments: Vec<Enchantment>,
    ) -> Self {
        Self {
            info,
            id: Uuid::new_v4().to_string(),
            level,
            rarity,
            health,
            defense,
            enchantments,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WeaponItem {
    pub info: ItemInfo,
    pub id: String,
    pub level: u32,
    pub rarity: ItemRarity,
    pub damage: u32,
    pub crit_hit_rate: f64,
    pub enchantments: Vec<Enchantment>,
}

impl WeaponItem {
    pub fn new(
        info: ItemInfo,
        level: u32,
        rarity: ItemRarity,
        damage: u32,
        crit_hit_rate: f64,
        enchantments: Vec<Enchantment>,
    ) -> Self {
        Self {
            info,
            id: Uuid::new_v4().to_string(),
            level,
            rarity,
            damage,
            crit_hit_rate,
            enchantments,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RingItem {
    pub info: ItemInfo,
    pub id: String,
    pub level: u32,
    pub rarity: ItemRarity,
    pub mana: u32,
    pub enchantments: Vec<Enchantment>,
}

impl RingItem {
    pub fn new(
        info: ItemInfo,
        level: u32,
        rarity: ItemRarity,
        mana: u32,
        enchantments: Vec<Enchantment>,
    ) -> Self {
        Self {
            info,
            id: Uuid::new_v4().to_string(),
            level,
            rarity,
            mana,
            enchantments,
        }
    }
}

pub struct ItemRarityDropRates {
    pub common: f64,
    pub uncommon: f64,
    pub rare: f64,
    pub epic: f64,
    pub legendary: f64,
}

pub struct WeaponBaseValues {
    pub min_damage: u32,
    pub max_damage: u32,
    pub min_crit_hit_rate: f64,
    pub max_crit_hit_rate: f64,
}

pub struct ArmorBaseValues {
    pub min_health: u32,
    pub max_health: u32,
    pub min_defense: u32,
    pub max_defense: u32,
}

pub struct RingBaseValues {
    pub min_mana: u32,
    pub max_mana: u32,
}

pub struct EnchantmentBaseValues {
    pub min_damage: u32,
    pub max_damage: u32,
    pub min_crit_hit_rate: f64,
    pub max_crit_hit_rate: f64,
    pub min_health: u32,
    pub max_health: u32,
    pub min_defense: u32,
    pub max_defense: u32,
    pub min_mana: u32,
    pub max_mana: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Enchantment {
    Damage(u32),
    CritHitRate(f64),
    Health(u32),
    Defense(u32),
    Mana(u32),
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ItemCategory {
    Consumable,
    Weapon,
    Armor,
    Ring,
    Unknown,
}

/// Returns the effect percentage of potions.
/// For example, returns 50 if the percentage is 50%.
/// 50 can be divided by 100 to get the decimal for calculations: 50/100 = 0.5.
/// E.g. for health potions, the amount of restored health is then 0.5 * MAX_HEALTH.
pub fn get_potion_effect_percentage(rarity: &ItemRarity) -> i32 {
    match rarity {
        ItemRarity::Common => 20,
        ItemRarity::Uncommon => 40,
        ItemRarity::Rare => 60,
        ItemRarity::Epic => 80,
        ItemRarity::Legendary => 100,
        _ => 0,
    }
}

pub fn get_health_potion_effect(rarity: &ItemRarity) -> String {
    format!(
        "Restores {}% of your maximum health points.",
        get_potion_effect_percentage(rarity)
    )
}

pub fn get_consumable_full_name(name: &str, rarity: &ItemRarity) -> String {
    format!("{:?} {}", rarity, name)
}

pub fn create_starter_weapon() -> WeaponItem {
    WeaponItem::new(
        ITEM_SWORD,
        1,
        ItemRarity::Common,
        WEAPON_BASE_VALUES.min_damage,
        WEAPON_BASE_VALUES.min_crit_hit_rate,
        Vec::new(),
    )
}

pub fn random_equipment_item() -> ItemCategory {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0..=2);
    match rand_num {
        0 => ItemCategory::Weapon,
        1 => ItemCategory::Armor,
        2 => ItemCategory::Ring,
        _ => ItemCategory::Unknown,
    }
}

pub fn random_item_rarity(drop_rates: &ItemRarityDropRates) -> ItemRarity {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0.0..1.0);
    let mut drop_rate = 0.0;

    drop_rate += drop_rates.common;
    if rand_num < drop_rate {
        return ItemRarity::Common;
    }

    drop_rate += drop_rates.uncommon;
    if rand_num < drop_rate {
        return ItemRarity::Uncommon;
    }

    drop_rate += drop_rates.rare;
    if rand_num < drop_rate {
        return ItemRarity::Rare;
    }

    drop_rate += drop_rates.epic;
    if rand_num < drop_rate {
        return ItemRarity::Epic;
    }

    drop_rate += drop_rates.legendary;
    if rand_num < drop_rate {
        return ItemRarity::Legendary;
    }

    ItemRarity::Unknown
}

pub fn num_enchantments(rarity: &ItemRarity) -> u8 {
    match rarity {
        ItemRarity::Common => 0,
        ItemRarity::Uncommon => 1,
        ItemRarity::Rare => 2,
        ItemRarity::Epic => 3,
        ItemRarity::Legendary => 4,
        _ => 0,
    }
}

pub fn generate_item_enchantments(
    num: u8,
    category: ItemCategory,
    base_values: &EnchantmentBaseValues,
    dungeon_floor: u32,
) -> Vec<Enchantment> {
    let mut enchantments: Vec<Enchantment> = Vec::new();
    for _ in 0..num {
        match category {
            ItemCategory::Weapon => {
                enchantments.push(random_weapon_enchantment(base_values, dungeon_floor))
            }
            ItemCategory::Armor => {
                enchantments.push(random_armor_enchantment(base_values, dungeon_floor))
            }
            ItemCategory::Ring => {
                enchantments.push(random_ring_enchantment(base_values, dungeon_floor))
            }
            _ => {}
        }
    }
    enchantments
}

pub fn random_weapon_enchantment(
    base_values: &EnchantmentBaseValues,
    dungeon_floor: u32,
) -> Enchantment {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(0..=1);
    match rand_num {
        0 => {
            let damage = rng.gen_range(base_values.min_damage..=base_values.max_damage)
                + (3 * dungeon_floor);
            return Enchantment::Damage(damage);
        }
        1 => {
            let crit_hit_rate =
                rng.gen_range(base_values.min_crit_hit_rate..=base_values.max_crit_hit_rate);
            return Enchantment::CritHitRate(crit_hit_rate);
        }
        _ => Enchantment::Unknown,
    }
}

pub fn random_armor_enchantment(
    base_values: &EnchantmentBaseValues,
    dungeon_floor: u32,
) -> Enchantment {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(0..=1);
    match rand_num {
        0 => {
            let health = rng.gen_range(base_values.min_health..=base_values.max_health)
                + (5 * dungeon_floor);
            return Enchantment::Health(health);
        }
        1 => {
            let defense = rng.gen_range(base_values.min_defense..=base_values.max_defense)
                + (1 * dungeon_floor);
            return Enchantment::Defense(defense);
        }
        _ => Enchantment::Unknown,
    }
}

pub fn random_ring_enchantment(
    base_values: &EnchantmentBaseValues,
    dungeon_floor: u32,
) -> Enchantment {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(0..=3);
    match rand_num {
        0 => {
            let mana = rng.gen_range(base_values.min_mana..=base_values.max_mana);
            return Enchantment::Mana(mana);
        }
        1 => {
            let damage = rng.gen_range(base_values.min_damage..=base_values.max_damage)
                + (3 * dungeon_floor);
            return Enchantment::Damage(damage);
        }
        2 => {
            let health = rng.gen_range(base_values.min_health..=base_values.max_health)
                + (5 * dungeon_floor);
            return Enchantment::Health(health);
        }
        3 => {
            let crit_hit_rate =
                rng.gen_range(base_values.min_crit_hit_rate..=base_values.max_crit_hit_rate);
            return Enchantment::CritHitRate(crit_hit_rate);
        }
        _ => Enchantment::Unknown,
    }
}

pub fn generate_random_weapon(base_values: WeaponBaseValues, dungeon_floor: u32) -> WeaponItem {
    let mut rng = thread_rng();
    let damage =
        rng.gen_range(base_values.min_damage..=base_values.max_damage) + (5 * dungeon_floor);
    let crit_hit_rate =
        rng.gen_range(base_values.min_crit_hit_rate..=base_values.max_crit_hit_rate);
    let rarity = random_item_rarity(&ITEM_RARITY_DROP_RATES);
    let enchantments = generate_item_enchantments(
        num_enchantments(&rarity),
        ItemCategory::Weapon,
        &ENCHANTMENT_BASE_VALUES,
        dungeon_floor,
    );

    WeaponItem::new(
        ITEM_SWORD,
        dungeon_floor,
        rarity,
        damage,
        crit_hit_rate,
        enchantments,
    )
}

pub fn generate_random_armor(base_values: ArmorBaseValues, dungeon_floor: u32) -> ArmorItem {
    let mut rng = thread_rng();
    let health =
        rng.gen_range(base_values.min_health..=base_values.max_health) + (10 * dungeon_floor);
    let defense =
        rng.gen_range(base_values.min_defense..=base_values.max_defense) + (2 * dungeon_floor);
    let rarity = random_item_rarity(&ITEM_RARITY_DROP_RATES);
    let enchantments = generate_item_enchantments(
        num_enchantments(&rarity),
        ItemCategory::Armor,
        &ENCHANTMENT_BASE_VALUES,
        dungeon_floor,
    );

    ArmorItem::new(
        ITEM_ARMOR,
        dungeon_floor,
        rarity,
        health,
        defense,
        enchantments,
    )
}

pub fn generate_random_ring(base_values: RingBaseValues, dungeon_floor: u32) -> RingItem {
    let mut rng = thread_rng();
    let mana = rng.gen_range(base_values.min_mana..=base_values.max_mana);
    let rarity = random_item_rarity(&ITEM_RARITY_DROP_RATES);
    let enchantments = generate_item_enchantments(
        num_enchantments(&rarity),
        ItemCategory::Ring,
        &ENCHANTMENT_BASE_VALUES,
        dungeon_floor,
    );

    RingItem::new(ITEM_RING, dungeon_floor, rarity, mana, enchantments)
}
