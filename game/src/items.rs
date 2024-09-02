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
pub const WEAPON_BASE_STATS: WeaponBaseStats = WeaponBaseStats {
    min_damage: 12,
    max_damage: 15,
    min_crit_hit_rate: 0.15,
    max_crit_hit_rate: 0.20,
};
pub const ARMOR_BASE_STATS: ArmorBaseStats = ArmorBaseStats {
    min_health: 20,
    max_health: 25,
    min_defense: 1,
    max_defense: 3,
};
pub const RING_BASE_STATS: RingBaseStats = RingBaseStats {
    min_mana: 20,
    max_mana: 25,
};
pub const ENCHANTMENT_BASE_STATS: EnchantmentBaseStats = EnchantmentBaseStats {
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

pub struct WeaponBaseStats {
    pub min_damage: u32,
    pub max_damage: u32,
    pub min_crit_hit_rate: f64,
    pub max_crit_hit_rate: f64,
}

pub struct ArmorBaseStats {
    pub min_health: u32,
    pub max_health: u32,
    pub min_defense: u32,
    pub max_defense: u32,
}

pub struct RingBaseStats {
    pub min_mana: u32,
    pub max_mana: u32,
}

pub struct EnchantmentBaseStats {
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
        damage: WEAPON_BASE_STATS.min_damage,
        crit_hit_rate: WEAPON_BASE_STATS.min_crit_hit_rate,
        enchantments: Vec::new(),
    }
}
