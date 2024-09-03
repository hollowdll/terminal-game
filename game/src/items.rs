use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

pub const ITEM_RARITY_DROP_RATE: ItemRarityDropRate = ItemRarityDropRate {
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
    min_crit_hit_rate: 0.05,
    max_crit_hit_rate: 0.08,
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
    category: ItemCategory::Disposable,
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
pub struct DisposableItem {
    pub info: ItemInfo,
    pub effect: String,
    pub rarity: ItemRarity,
    pub amount_in_inventory: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArmorItem {
    pub info: ItemInfo,
    pub global_id: String,
    pub rarity: ItemRarity,
    pub health: u32,
    pub defense: u32,
    pub enchantments: Vec<Enchantment>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WeaponItem {
    pub info: ItemInfo,
    pub global_id: String,
    pub rarity: ItemRarity,
    pub damage: u32,
    pub crit_hit_rate: f64,
    pub enchantments: Vec<Enchantment>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RingItem {
    pub info: ItemInfo,
    pub global_id: String,
    pub rarity: ItemRarity,
    pub mana: u32,
    pub enchantments: Vec<Enchantment>,
}

pub struct ItemRarityDropRate {
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
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ItemRarity {
    /// No enchantments.
    Common,
    /// 1 enchantment.
    Uncommon,
    /// 2 enchantments.
    Rare,
    /// 3 enchantments.
    Epic,
    /// 4 enchantments.
    Legendary,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ItemCategory {
    Disposable,
    Weapon,
    Armor,
    Ring,
}

/// Returns the effect percentage of potions.
/// For example, returns 50 if the percentage is 50%.
/// 50 can be divided by 100 to get the decimal for calculations: 50/100 = 0.5.
/// E.g. for health potions, the amount of restored health is then 0.5 * MAX_HEALTH.
pub fn get_potion_effect_percentage(rarity: ItemRarity) -> i32 {
    match rarity {
        ItemRarity::Common => 20,
        ItemRarity::Uncommon => 35,
        ItemRarity::Rare => 50,
        ItemRarity::Epic => 75,
        ItemRarity::Legendary => 100,
    }
}

pub fn create_starter_weapon() -> WeaponItem {
    WeaponItem {
        info: ITEM_SWORD,
        global_id: Uuid::new_v4().to_string(),
        rarity: ItemRarity::Common,
        damage: WEAPON_BASE_VALUES.min_damage,
        crit_hit_rate: WEAPON_BASE_VALUES.min_crit_hit_rate,
        enchantments: Vec::new(),
    }
}

pub fn random_equipment_item() -> Option<ItemCategory> {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0..=2);
    match rand_num {
        0 => Some(ItemCategory::Weapon),
        1 => Some(ItemCategory::Armor),
        2 => Some(ItemCategory::Ring),
        _ => None,
    }
}

pub fn random_item_rarity(drop_rates: ItemRarityDropRate) -> Option<ItemRarity> {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0.0..1.0);
    let mut drop_rate = 0.0;

    drop_rate += drop_rates.common;
    if rand_num < drop_rate {
        return Some(ItemRarity::Common);
    }

    drop_rate += drop_rates.uncommon;
    if rand_num < drop_rate {
        return Some(ItemRarity::Uncommon);
    }

    drop_rate += drop_rates.rare;
    if rand_num < drop_rate {
        return Some(ItemRarity::Rare);
    }

    drop_rate += drop_rates.epic;
    if rand_num < drop_rate {
        return Some(ItemRarity::Epic);
    }

    drop_rate += drop_rates.legendary;
    if rand_num < drop_rate {
        return Some(ItemRarity::Legendary);
    }

    None
}

pub fn random_weapon_enchantment(base_values: EnchantmentBaseValues) -> Option<Enchantment> {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(0..=1);
    match rand_num {
        0 => {
            let rand_damage = rng.gen_range(base_values.min_damage..=base_values.max_damage);
            return Some(Enchantment::Damage(rand_damage));
        }
        1 => {
            let rand_crit_hit_rate =
                rng.gen_range(base_values.min_crit_hit_rate..=base_values.max_crit_hit_rate);
            return Some(Enchantment::CritHitRate(rand_crit_hit_rate));
        }
        _ => None,
    }
}

pub fn generate_random_weapon() -> Option<WeaponItem> {
    None
}
