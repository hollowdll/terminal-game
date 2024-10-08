use crate::{
    character::CharacterClass,
    items::{
        generate_random_armor, generate_random_ring, generate_random_weapon, get_item_display_name,
        get_item_purchase_value, get_item_sell_value, random_item_rarity, ArmorItem, CharacterItem,
        ConsumableItem, ItemRarity, RingItem, WeaponItem, ARMOR_BASE_VALUES,
        ITEM_RARITY_DROP_RATES, RING_BASE_VALUES, WEAPON_BASE_VALUES,
    },
    session::PlayerCharacter,
};

pub struct ShopItems {
    pub consumables: Vec<ConsumableItem>,
    pub weapon: Option<WeaponItem>,
    pub armor: Option<ArmorItem>,
    pub ring: Option<RingItem>,
}

impl ShopItems {
    /// Returns true if player has enough gold and the item was bought.
    pub fn buy_weapon(&mut self, character: &mut PlayerCharacter) -> bool {
        if let Some(item) = &self.weapon {
            let purchase_value = get_item_purchase_value(&item.rarity);
            if character.data.currency.gold >= purchase_value {
                character.give_weapon(&item);
                character.data.currency.gold -= purchase_value;
                self.weapon = None;
                return true;
            }
        }
        false
    }

    /// Returns true if player has enough gold and the item was bought.
    pub fn buy_armor(&mut self, character: &mut PlayerCharacter) -> bool {
        if let Some(item) = &self.armor {
            let purchase_value = get_item_purchase_value(&item.rarity);
            if character.data.currency.gold >= purchase_value {
                character.give_armor(&item);
                character.data.currency.gold -= purchase_value;
                self.armor = None;
                return true;
            }
        }
        false
    }

    /// Returns true if player has enough gold and the item was bought.
    pub fn buy_ring(&mut self, character: &mut PlayerCharacter) -> bool {
        if let Some(item) = &self.ring {
            let purchase_value = get_item_purchase_value(&item.rarity);
            if character.data.currency.gold >= purchase_value {
                character.give_ring(&item);
                character.data.currency.gold -= purchase_value;
                self.ring = None;
                return true;
            }
        }
        false
    }
}

pub fn generate_shop_items(floor: u32, class: &CharacterClass) -> ShopItems {
    ShopItems {
        consumables: vec![
            ConsumableItem::new_health_potion(ItemRarity::Common),
            ConsumableItem::new_health_potion(ItemRarity::Uncommon),
            ConsumableItem::new_health_potion(ItemRarity::Rare),
            ConsumableItem::new_health_potion(ItemRarity::Epic),
            ConsumableItem::new_health_potion(ItemRarity::Legendary),
            ConsumableItem::new_mana_potion(ItemRarity::Common),
            ConsumableItem::new_mana_potion(ItemRarity::Uncommon),
            ConsumableItem::new_mana_potion(ItemRarity::Rare),
            ConsumableItem::new_mana_potion(ItemRarity::Epic),
            ConsumableItem::new_mana_potion(ItemRarity::Legendary),
        ],
        weapon: Some(generate_random_weapon(
            random_item_rarity(&ITEM_RARITY_DROP_RATES),
            WEAPON_BASE_VALUES,
            floor,
            class,
        )),
        armor: Some(generate_random_armor(
            random_item_rarity(&ITEM_RARITY_DROP_RATES),
            ARMOR_BASE_VALUES,
            floor,
        )),
        ring: Some(generate_random_ring(
            random_item_rarity(&ITEM_RARITY_DROP_RATES),
            RING_BASE_VALUES,
            floor,
        )),
    }
}

/// Returns true if player has enough gold and the item was bought.
pub fn buy_consumable(item: &ConsumableItem, character: &mut PlayerCharacter) -> bool {
    let purchase_value = get_item_purchase_value(&item.rarity);
    if character.data.currency.gold >= purchase_value {
        character.give_consumable(&item, 1);
        character.data.currency.gold -= purchase_value;
        return true;
    }
    false
}

/// Returns the amount of gold received.
pub fn sell_consumable(
    consumable: &ConsumableItem,
    amount: u32,
    character: &mut PlayerCharacter,
) -> u32 {
    let name = get_item_display_name(CharacterItem::Consumable(consumable));
    let sell_value = get_item_sell_value(&consumable.rarity);
    if consumable.amount_in_inventory > amount {
        if character.decrease_consumable_inventory_amount(&name, amount) {
            let gold = amount * sell_value;
            character.give_gold(gold);
            return gold;
        }
    } else if character.delete_consumable(&name) {
        let gold = consumable.amount_in_inventory * sell_value;
        character.give_gold(gold);
        return gold;
    }
    0
}

/// Returns the amount of gold received.
pub fn sell_weapon(weapon: &WeaponItem, character: &mut PlayerCharacter) -> u32 {
    if character.delete_weapon(&weapon.id) {
        let gold = get_item_sell_value(&weapon.rarity);
        character.give_gold(gold);
        return gold;
    }
    0
}

/// Returns the amount of gold received.
pub fn sell_armor(armor: &ArmorItem, character: &mut PlayerCharacter) -> u32 {
    if character.delete_armor(&armor.id) {
        let gold = get_item_sell_value(&armor.rarity);
        character.give_gold(gold);
        return gold;
    }
    0
}

/// Returns the amount of gold received.
pub fn sell_ring(ring: &RingItem, character: &mut PlayerCharacter) -> u32 {
    if character.delete_ring(&ring.id) {
        let gold = get_item_sell_value(&ring.rarity);
        character.give_gold(gold);
        return gold;
    }
    0
}
