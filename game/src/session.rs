use crate::{
    game_data::{CharacterData, GameData},
    items::{
        generate_random_armor, generate_random_ring, generate_random_weapon, get_item_display_name,
        ArmorItem, CharacterItem, ConsumableItem, ItemRarity, RingItem, WeaponItem,
        ARMOR_BASE_VALUES, RING_BASE_VALUES, WEAPON_BASE_VALUES,
    },
};

pub struct Player {
    pub character: Option<PlayerCharacter>,
    pub data: GameData,
}

impl Player {
    pub fn new(data: GameData) -> Self {
        Self {
            character: None,
            data,
        }
    }
}

pub struct PlayerCharacter {
    pub data: CharacterData,
    pub temp_stats: TemporaryStats,
    pub temp_stat_boosts: TemporaryStatBoosts,
}

impl PlayerCharacter {
    pub fn new(data: &CharacterData) -> Self {
        Self {
            data: data.clone(),
            temp_stats: TemporaryStats {
                health: data.stats.combat_stats.max_health,
                mana: data.stats.combat_stats.max_mana,
            },
            temp_stat_boosts: TemporaryStatBoosts {
                max_health: 0,
                max_mana: 0,
                defense: 0,
                damage: 0,
                critical_damage_multiplier: 0.0,
                critical_hit_rate: 0.0,
            },
        }
    }

    pub fn give_gold(&mut self, amount: u64) {
        self.data.currency.gold += amount;
    }

    pub fn give_consumable(&mut self, item: &ConsumableItem, amount: u32) {
        if let Some(item) = self
            .data
            .inventory
            .consumables
            .get_mut(&get_item_display_name(CharacterItem::Consumable(&item)))
        {
            item.amount_in_inventory += amount;
        } else {
            self.data.inventory.consumables.insert(
                get_item_display_name(CharacterItem::Consumable(&item)),
                ConsumableItem {
                    info: item.info.clone(),
                    effect: item.effect.clone(),
                    rarity: item.rarity.clone(),
                    amount_in_inventory: amount,
                },
            );
        }
    }

    pub fn give_weapon(&mut self, item: &WeaponItem) {
        self.data
            .inventory
            .weapons
            .insert(item.id.clone(), item.clone());
    }

    pub fn give_armor(&mut self, item: &ArmorItem) {
        self.data
            .inventory
            .armors
            .insert(item.id.clone(), item.clone());
    }

    pub fn give_ring(&mut self, item: &RingItem) {
        self.data
            .inventory
            .rings
            .insert(item.id.clone(), item.clone());
    }

    /// The returned bool is true if the weapon is in the inventory and it was equipped.
    /// If the item doesn't exist in the inventory, the returned bool is false.
    pub fn equip_weapon(&mut self, weapon_id: &str) -> bool {
        match self.data.inventory.weapons.get(weapon_id) {
            Some(weapon) => {
                self.data.equipment.weapon = Some(weapon.clone());
                // TODO increase stats
                return true;
            }
            None => return false,
        }
    }

    pub fn unequip_weapon(&mut self) -> bool {
        if let Some(_weapon) = &self.data.equipment.weapon {
            // TODO decrease stats
            self.data.equipment.weapon = None;
            return true;
        }
        false
    }

    pub fn gain_exp(&mut self, exp: u32) {
        self.data.stats.general_stats.current_exp += exp;

        while self.data.stats.general_stats.current_exp
            >= self.data.stats.general_stats.required_exp
        {
            self.level_up();
        }
    }

    pub fn level_up(&mut self) -> u32 {
        self.data.stats.general_stats.character_level += 1;
        self.data.stats.general_stats.current_exp -= self.data.stats.general_stats.required_exp;
        self.data.stats.general_stats.required_exp =
            (self.data.stats.general_stats.required_exp as f32 * 1.1).round() as u32;

        // increase max health and damage on level up
        self.data.stats.combat_stats.max_health += 10;
        self.data.stats.combat_stats.damage += 3;

        return self.data.stats.general_stats.character_level;
    }

    pub fn give_test_items(&mut self) {
        self.give_consumable(&ConsumableItem::new_health_potion(ItemRarity::Common), 5);
        self.give_consumable(&ConsumableItem::new_health_potion(ItemRarity::Uncommon), 4);
        self.give_consumable(&ConsumableItem::new_health_potion(ItemRarity::Rare), 3);
        self.give_consumable(&ConsumableItem::new_health_potion(ItemRarity::Epic), 2);
        self.give_consumable(&ConsumableItem::new_health_potion(ItemRarity::Legendary), 1);
        self.give_armor(&generate_random_armor(ARMOR_BASE_VALUES, 1));
        self.give_armor(&generate_random_armor(ARMOR_BASE_VALUES, 1));
        self.give_armor(&generate_random_armor(ARMOR_BASE_VALUES, 1));
        self.give_ring(&generate_random_ring(RING_BASE_VALUES, 1));
        self.give_ring(&generate_random_ring(RING_BASE_VALUES, 1));
        self.give_weapon(&generate_random_weapon(WEAPON_BASE_VALUES, 1));
    }

    pub fn delete_consumable(&mut self, display_name: &str) -> bool {
        if let Some(_) = self.data.inventory.consumables.remove(display_name) {
            return true;
        }
        false
    }

    /// Returns true if the item exists in inventory.
    pub fn decrease_consumable_inventory_amount(
        &mut self,
        display_name: &str,
        amount: u32,
    ) -> bool {
        if let Some(item) = self.data.inventory.consumables.get_mut(display_name) {
            if amount > item.amount_in_inventory {
                item.amount_in_inventory = 0;
            } else {
                item.amount_in_inventory -= amount;
            }
            return true;
        }
        false
    }

    pub fn delete_weapon(&mut self, id: &str) -> bool {
        if let Some(_) = self.data.inventory.weapons.remove(id) {
            if let Some(weapon) = &self.data.equipment.weapon {
                if weapon.id.eq(id) {
                    self.unequip_weapon();
                }
            }
            return true;
        }
        false
    }
}

pub struct TemporaryStats {
    pub health: u32,
    pub mana: u32,
}

pub struct TemporaryStatBoosts {
    pub max_health: u32,
    pub max_mana: u32,
    pub defense: u32,
    pub damage: u32,
    pub critical_damage_multiplier: f64,
    pub critical_hit_rate: f64,
}
