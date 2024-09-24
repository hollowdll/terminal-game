use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

use crate::{character::CharacterClass, session::PlayerCharacter};

pub const ITEM_RARITY_DROP_RATES: ItemRarityDropRates = ItemRarityDropRates {
    common: 0.43,
    uncommon: 0.30,
    rare: 0.14,
    epic: 0.08,
    legendary: 0.05,
};
pub const WEAPON_BASE_VALUES: WeaponBaseValues = WeaponBaseValues {
    min_damage: 12,
    max_damage: 15,
    min_crit_hit_rate: 0.12,
    max_crit_hit_rate: 0.15,
};
pub const ARMOR_BASE_VALUES: ArmorBaseValues = ArmorBaseValues {
    min_health: 15,
    max_health: 20,
    min_defense: 1,
    max_defense: 2,
};
pub const RING_BASE_VALUES: RingBaseValues = RingBaseValues {
    min_mana: 20,
    max_mana: 25,
};
pub const ENCHANTMENT_BASE_VALUES: EnchantmentBaseValues = EnchantmentBaseValues {
    min_damage: 4,
    max_damage: 6,
    min_crit_hit_rate: 0.03,
    max_crit_hit_rate: 0.05,
    min_health: 7,
    max_health: 10,
    min_defense: 1,
    max_defense: 2,
    min_mana: 10,
    max_mana: 15,
};

//-------------------//
// Consumable items //
//-----------------//

pub const ITEM_HEALTH_POTION_NAME: &str = "Health Potion";
pub const ITEM_HEALTH_POTION: ItemInfo = ItemInfo {
    name: Cow::Borrowed(ITEM_HEALTH_POTION_NAME),
    description: Cow::Borrowed("A magical potion that restores health points."),
    category: ItemCategory::Consumable,
};

pub const ITEM_MANA_POTION_NAME: &str = "Mana Potion";
pub const ITEM_MANA_POTION: ItemInfo = ItemInfo {
    name: Cow::Borrowed(ITEM_MANA_POTION_NAME),
    description: Cow::Borrowed("A magical potion that restores mana points."),
    category: ItemCategory::Consumable,
};

//---------------//
// Weapon items //
//-------------//

pub const WEAPON_NAME_ANCIENT_KNIGHT: &str = "Greatsword";

pub const ITEM_SWORD: ItemInfo = ItemInfo {
    name: Cow::Borrowed("Sword"),
    description: Cow::Borrowed("A sword that increases offensive stats."),
    category: ItemCategory::Weapon,
};

pub const ITEM_AXE: ItemInfo = ItemInfo {
    name: Cow::Borrowed("Axe"),
    description: Cow::Borrowed("An axe that increases offensive stats."),
    category: ItemCategory::Weapon,
};

pub const ITEM_STAFF: ItemInfo = ItemInfo {
    name: Cow::Borrowed("Staff"),
    description: Cow::Borrowed("A staff that increases offensive stats."),
    category: ItemCategory::Weapon,
};

pub const ITEM_DAGGER: ItemInfo = ItemInfo {
    name: Cow::Borrowed("Dagger"),
    description: Cow::Borrowed("A dagger that increases offensive stats."),
    category: ItemCategory::Weapon,
};

pub const ITEM_HALBERD: ItemInfo = ItemInfo {
    name: Cow::Borrowed("Halberd"),
    description: Cow::Borrowed("A halberd that increases offensive stats."),
    category: ItemCategory::Weapon,
};

pub const ITEM_DIVINE_GREATSWORD: ItemInfo = ItemInfo {
    name: Cow::Borrowed(WEAPON_NAME_ANCIENT_KNIGHT),
    description: Cow::Borrowed("A mythical greatsword that increases offensive stats."),
    category: ItemCategory::Weapon,
};

//--------------//
// Armor items //
//------------//

pub const ITEM_ARMOR: ItemInfo = ItemInfo {
    name: Cow::Borrowed("Armor"),
    description: Cow::Borrowed("An armor that increases defensive stats."),
    category: ItemCategory::Armor,
};

//-------------//
// Ring items //
//-----------//

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

    pub fn new_mana_potion(rarity: ItemRarity) -> Self {
        Self {
            info: ITEM_MANA_POTION,
            effect: get_mana_potion_effect(&rarity),
            rarity,
            amount_in_inventory: 0,
        }
    }

    /// Returns text telling what the item did.
    pub fn use_item(&self, character: &mut PlayerCharacter) -> String {
        let display_name = get_item_display_name(CharacterItem::Consumable(&self));
        let mut text = "Player used an unknown item".to_string();
        match self.info.name.as_ref() {
            ITEM_HEALTH_POTION_NAME => {
                let heal_percentage = get_potion_effect_percentage(&self.rarity) as f64 / 100.0;
                let restored_health = character
                    .restore_health((heal_percentage * character.get_total_health() as f64) as u32);
                text = format!(
                    "Player used {}! Player restored {} health points",
                    &display_name, restored_health
                );
            }
            ITEM_MANA_POTION_NAME => {
                let heal_percentage = get_potion_effect_percentage(&self.rarity) as f64 / 100.0;
                let restored_mana = character
                    .restore_mana((heal_percentage * character.get_total_mana() as f64) as u32);
                text = format!(
                    "Player used {}! Player restored {} mana points",
                    &display_name, restored_mana
                );
            }
            _ => {}
        }
        if self.amount_in_inventory > 1 {
            character.decrease_consumable_inventory_amount(&display_name, 1);
        } else {
            character.delete_consumable(&display_name);
        }
        text
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArmorItemStats {
    pub health: u32,
    pub defense: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArmorItem {
    pub info: ItemInfo,
    pub id: String,
    pub level: u32,
    pub rarity: ItemRarity,
    pub stats: ArmorItemStats,
    pub enchantments: Vec<Enchantment>,
}

impl ArmorItem {
    pub fn new(
        info: ItemInfo,
        level: u32,
        rarity: ItemRarity,
        stats: ArmorItemStats,
        enchantments: Vec<Enchantment>,
    ) -> Self {
        Self {
            info,
            id: Uuid::new_v4().to_string(),
            level,
            rarity,
            stats,
            enchantments,
        }
    }

    pub fn is_equipped(&self, character: &PlayerCharacter) -> bool {
        if let Some(id) = &character.equipped_items.armor {
            if let Some(armor) = character.data.inventory.armors.get(id) {
                if armor.id.eq(&self.id) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WeaponItemStats {
    pub damage: u32,
    pub crit_hit_rate: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WeaponItem {
    pub info: ItemInfo,
    pub id: String,
    pub level: u32,
    pub rarity: ItemRarity,
    pub stats: WeaponItemStats,
    pub enchantments: Vec<Enchantment>,
}

impl WeaponItem {
    pub fn new(
        info: ItemInfo,
        level: u32,
        rarity: ItemRarity,
        stats: WeaponItemStats,
        enchantments: Vec<Enchantment>,
    ) -> Self {
        Self {
            info,
            id: Uuid::new_v4().to_string(),
            level,
            rarity,
            stats,
            enchantments,
        }
    }

    pub fn is_equipped(&self, character: &PlayerCharacter) -> bool {
        if let Some(id) = &character.equipped_items.weapon {
            if let Some(weapon) = character.data.inventory.weapons.get(id) {
                if weapon.id.eq(&self.id) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RingItemStats {
    pub mana: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RingItem {
    pub info: ItemInfo,
    pub id: String,
    pub level: u32,
    pub rarity: ItemRarity,
    pub stats: RingItemStats,
    pub enchantments: Vec<Enchantment>,
}

impl RingItem {
    pub fn new(
        info: ItemInfo,
        level: u32,
        rarity: ItemRarity,
        stats: RingItemStats,
        enchantments: Vec<Enchantment>,
    ) -> Self {
        Self {
            info,
            id: Uuid::new_v4().to_string(),
            level,
            rarity,
            stats,
            enchantments,
        }
    }

    pub fn is_equipped(&self, character: &PlayerCharacter) -> bool {
        if let Some(id) = &character.equipped_items.ring {
            if let Some(ring) = character.data.inventory.rings.get(id) {
                if ring.id.eq(&self.id) {
                    return true;
                }
            }
        }
        false
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

pub enum CharacterItem<'a> {
    Consumable(&'a ConsumableItem),
    Weapon(&'a WeaponItem),
    Armor(&'a ArmorItem),
    Ring(&'a RingItem),
    Unknown,
}

pub enum CharacterItemOwned {
    Consumable(ConsumableItem),
    Weapon(WeaponItem),
    Armor(ArmorItem),
    Ring(RingItem),
    Unknown,
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
    Mythical,
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

pub fn get_mana_potion_effect(rarity: &ItemRarity) -> String {
    format!(
        "Restores {}% of your maximum mana points.",
        get_potion_effect_percentage(rarity)
    )
}

/// Returns a string representation of an item.
/// The string is used to display the item in menus.
pub fn get_item_display_name<'a>(item: CharacterItem<'a>) -> String {
    match item {
        CharacterItem::Consumable(consumable) => {
            format!("{:?} {}", consumable.rarity, consumable.info.name)
        }
        CharacterItem::Weapon(weapon) => {
            format!(
                "{:?} {} (Level {})",
                weapon.rarity, weapon.info.name, weapon.level
            )
        }
        CharacterItem::Armor(armor) => {
            format!(
                "{:?} {} (Level {})",
                armor.rarity, armor.info.name, armor.level
            )
        }
        CharacterItem::Ring(ring) => {
            format!(
                "{:?} {} (Level {})",
                ring.rarity, ring.info.name, ring.level
            )
        }
        _ => format!("?Unknown?"),
    }
}

/// Returns the purchase value of an item in gold.
pub fn get_item_purchase_value(rarity: &ItemRarity) -> u32 {
    match rarity {
        ItemRarity::Common => 100,
        ItemRarity::Uncommon => 250,
        ItemRarity::Rare => 400,
        ItemRarity::Epic => 550,
        ItemRarity::Legendary => 700,
        _ => 0,
    }
}

/// Returns the sell value of an item in gold.
pub fn get_item_sell_value(rarity: &ItemRarity) -> u32 {
    match rarity {
        ItemRarity::Common => 25,
        ItemRarity::Uncommon => 50,
        ItemRarity::Rare => 75,
        ItemRarity::Epic => 100,
        ItemRarity::Legendary => 125,
        _ => 0,
    }
}

pub fn create_starter_weapon(character_class: &CharacterClass) -> WeaponItem {
    let item_info = match character_class {
        CharacterClass::Mage => ITEM_STAFF,
        CharacterClass::Cleric => ITEM_HALBERD,
        CharacterClass::Assassin => ITEM_DAGGER,
        CharacterClass::Warrior => ITEM_AXE,
        CharacterClass::Knight => ITEM_SWORD,
    };
    WeaponItem::new(
        item_info,
        1,
        ItemRarity::Common,
        WeaponItemStats {
            damage: WEAPON_BASE_VALUES.min_damage,
            crit_hit_rate: WEAPON_BASE_VALUES.min_crit_hit_rate,
        },
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
                + (2 * dungeon_floor);
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
                + (4 * dungeon_floor);
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
                + (2 * dungeon_floor);
            return Enchantment::Damage(damage);
        }
        2 => {
            let health = rng.gen_range(base_values.min_health..=base_values.max_health)
                + (3 * dungeon_floor);
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

pub fn generate_random_weapon(
    base_values: WeaponBaseValues,
    dungeon_floor: u32,
    character_class: &CharacterClass,
) -> WeaponItem {
    let mut rng = thread_rng();
    let damage =
        rng.gen_range(base_values.min_damage..=base_values.max_damage) + (3 * dungeon_floor);
    let crit_hit_rate =
        rng.gen_range(base_values.min_crit_hit_rate..=base_values.max_crit_hit_rate);
    let rarity = random_item_rarity(&ITEM_RARITY_DROP_RATES);
    let enchantments = generate_item_enchantments(
        num_enchantments(&rarity),
        ItemCategory::Weapon,
        &ENCHANTMENT_BASE_VALUES,
        dungeon_floor,
    );
    let item_info = match character_class {
        CharacterClass::Mage => ITEM_STAFF,
        CharacterClass::Cleric => ITEM_HALBERD,
        CharacterClass::Assassin => ITEM_DAGGER,
        CharacterClass::Warrior => ITEM_AXE,
        CharacterClass::Knight => ITEM_SWORD,
    };

    WeaponItem::new(
        item_info,
        dungeon_floor,
        rarity,
        WeaponItemStats {
            damage,
            crit_hit_rate,
        },
        enchantments,
    )
}

pub fn generate_random_armor(base_values: ArmorBaseValues, dungeon_floor: u32) -> ArmorItem {
    let mut rng = thread_rng();
    let health =
        rng.gen_range(base_values.min_health..=base_values.max_health) + (8 * dungeon_floor);
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
        ArmorItemStats { health, defense },
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

    RingItem::new(
        ITEM_RING,
        dungeon_floor,
        rarity,
        RingItemStats { mana },
        enchantments,
    )
}

pub fn generate_random_consumable() -> ConsumableItem {
    let mut rng = thread_rng();
    let num = rng.gen_range(0..2);
    let rarity = random_item_rarity(&ITEM_RARITY_DROP_RATES);

    match num {
        0 => ConsumableItem::new_health_potion(rarity),
        1 => ConsumableItem::new_mana_potion(rarity),
        _ => ConsumableItem::new_health_potion(rarity),
    }
}

pub fn create_mythical_weapon(level: u32, item_info: ItemInfo) -> WeaponItem {
    let damage = 20 + (3 * level);
    let crit_hit_rate = 0.20;
    let mut enchantments: Vec<Enchantment> = Vec::new();

    match item_info.name.as_ref() {
        WEAPON_NAME_ANCIENT_KNIGHT => {
            enchantments.push(Enchantment::Damage(2 * level));
            enchantments.push(Enchantment::Damage(2 * level));
            enchantments.push(Enchantment::Damage(2 * level));
            enchantments.push(Enchantment::Damage(2 * level));
            enchantments.push(Enchantment::Damage(2 * level));
        }
        _ => {}
    }

    WeaponItem::new(
        item_info,
        level,
        ItemRarity::Mythical,
        WeaponItemStats {
            damage,
            crit_hit_rate,
        },
        enchantments,
    )
}
