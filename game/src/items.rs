use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    description: Cow::Borrowed("Medium damage, medium critical hit rate."),
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

#[derive(Serialize, Deserialize)]
pub struct ItemInfo {
    pub name: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub category: ItemCategory,
}

#[derive(Serialize, Deserialize)]
pub struct DisposableItem {
    pub info: ItemInfo,
    pub effect: String,
    pub rarity: ItemRarity,
    pub amount_in_inventory: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ArmorItem {
    pub info: ItemInfo,
    pub global_id: String,
    pub rarity: ItemRarity,
    pub base_health: u32,
    pub base_defense: u32,
}

#[derive(Serialize, Deserialize)]
pub struct WeaponItem {
    pub info: ItemInfo,
    pub global_id: String,
    pub rarity: ItemRarity,
    pub base_damage: u32,
    pub base_critical_hit_rate: f64,
}

#[derive(Serialize, Deserialize)]
pub struct RingItem {
    pub info: ItemInfo,
    pub global_id: String,
    pub rarity: ItemRarity,
    pub base_mana: u32,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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
        base_damage: 10,
        base_critical_hit_rate: 0.15,
    }
}
