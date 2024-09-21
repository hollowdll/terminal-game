use crate::{
    items::{
        generate_random_armor, generate_random_consumable, generate_random_ring,
        generate_random_weapon, get_item_display_name, get_item_sell_value, ArmorItem,
        CharacterItem, ConsumableItem, RingItem, WeaponItem, ARMOR_BASE_VALUES, RING_BASE_VALUES,
        WEAPON_BASE_VALUES,
    },
    session::PlayerCharacter,
};

pub struct ShopItems {
    pub consumable: ConsumableItem,
    pub weapon: WeaponItem,
    pub armor: ArmorItem,
    pub ring: RingItem,
}

pub fn randomize_shop_items(floor: u32) -> ShopItems {
    ShopItems {
        consumable: generate_random_consumable(),
        weapon: generate_random_weapon(WEAPON_BASE_VALUES, floor),
        armor: generate_random_armor(ARMOR_BASE_VALUES, floor),
        ring: generate_random_ring(RING_BASE_VALUES, floor),
    }
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
            return gold;
        }
    }
    if character.delete_consumable(&name) {
        let gold = consumable.amount_in_inventory * sell_value;
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
