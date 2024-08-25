//-------------------//
// Disposable items //
//-----------------//

pub const ITEM_HEALTH_POTION: ItemInfo = ItemInfo {
    name: "Health Potion",
    description: "A magical potion that restores health points.",
    category: ItemCategory::Disposable,
};

//---------------//
// Weapon items //
//-------------//

pub const ITEM_SWORD: ItemInfo = ItemInfo {
    name: "Sword",
    description: "Medium damage, medium critical hit rate.",
    category: ItemCategory::Weapon,
};

//--------------//
// Armor items //
//------------//

pub const ITEM_ARMOR: ItemInfo = ItemInfo {
    name: "Armor",
    description: "A defensive armor.",
    category: ItemCategory::Armor,
};

//--------//
// Rings //
//------//

pub const ITEM_RING: ItemInfo = ItemInfo {
    name: "Ring",
    description: "A ring that increases some stats.",
    category: ItemCategory::Ring,
};

//-----------------------------------//

pub struct ItemInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub category: ItemCategory,
}

pub struct DisposableItem {
    pub info: ItemInfo,
    pub effect: String,
    pub rarity: ItemRarity,
    pub amount_in_inventory: u32,
}

pub struct ArmorItem {
    pub info: ItemInfo,
    pub global_id: String,
    pub rarity: ItemRarity,
    pub base_health: u32,
    pub base_defense: u32,
}

pub struct WeaponItem {
    pub info: ItemInfo,
    pub global_id: String,
    pub rarity: ItemRarity,
    pub base_damage: u32,
    pub base_critical_hit_rate: f64,
}

pub struct RingItem {
    pub info: ItemInfo,
    pub global_id: String,
    pub rarity: ItemRarity,
    pub base_mana: u32,
}

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
