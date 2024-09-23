use std::io;

use crate::{
    character::{
        get_character_skill, CharacterClass, CharacterSkill, CLASS_ASSASSIN_STARTING_STATS,
        CLASS_CLERIC_STARTING_STATS, CLASS_KNIGHT_STARTING_STATS, CLASS_MAGE_STARTING_STATS,
        CLASS_WARRIOR_STARTING_STATS,
    },
    enemy::Enemy,
    fight::is_critical_hit,
    game_data::{CharacterData, GameData, STARTING_REQUIRED_EXP},
    items::{
        create_starter_weapon, generate_random_armor, generate_random_ring, generate_random_weapon,
        get_item_display_name, ArmorItem, CharacterItem, ConsumableItem, Enchantment, ItemRarity,
        RingItem, WeaponItem, ARMOR_BASE_VALUES, RING_BASE_VALUES, WEAPON_BASE_VALUES,
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

    /// Returns a reference to the player character.
    pub fn get_character(&self) -> io::Result<&PlayerCharacter> {
        match &self.character {
            Some(character) => Ok(character),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No selected character",
            )),
        }
    }

    /// Returns a mutable reference to the player character.
    pub fn get_character_mut(&mut self) -> io::Result<&mut PlayerCharacter> {
        match &mut self.character {
            Some(character) => Ok(character),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No selected character",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PlayerCharacter {
    pub data: CharacterData,
    pub temp_stats: TemporaryStats,
    pub temp_stat_boosts: TemporaryStatBoosts,
    pub equipped_items: EquippedItems,
}

impl PlayerCharacter {
    pub fn new(data: &CharacterData) -> Self {
        Self {
            data: data.clone(),
            temp_stats: TemporaryStats {
                current_health: data.stats.combat_stats.max_health,
                current_mana: data.stats.combat_stats.max_mana,
            },
            temp_stat_boosts: TemporaryStatBoosts {
                max_health: 0,
                max_mana: 0,
                defense: 0,
                damage: 0,
                critical_damage_multiplier: 0.0,
                critical_hit_rate: 0.0,
            },
            equipped_items: EquippedItems {
                weapon: None,
                armor: None,
                ring: None,
            },
        }
    }

    pub fn give_gold(&mut self, amount: u32) {
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

    /// Returns true if the item was equipped.
    pub fn equip_weapon(&mut self, id: &str) -> bool {
        self.unequip_weapon();
        if let Some(weapon) = self.data.inventory.weapons.get(id) {
            self.equipped_items.weapon = Some(id.to_string());
            self.temp_stat_boosts.increase_damage(weapon.stats.damage);
            self.temp_stat_boosts
                .increase_crit_hit_rate(weapon.stats.crit_hit_rate);
            self.temp_stat_boosts
                .give_enchantment_values(&weapon.enchantments);
            return true;
        }
        false
    }

    /// Returns true if the item was equipped.
    pub fn equip_armor(&mut self, id: &str) -> bool {
        self.unequip_armor();
        if let Some(armor) = self.data.inventory.armors.get(id) {
            self.equipped_items.armor = Some(id.to_string());
            self.temp_stat_boosts
                .increase_max_health(armor.stats.health);
            self.temp_stat_boosts.increase_defense(armor.stats.defense);
            self.temp_stat_boosts
                .give_enchantment_values(&armor.enchantments);
            return true;
        }
        false
    }

    /// Returns true if the item was equipped.
    pub fn equip_ring(&mut self, id: &str) -> bool {
        self.unequip_ring();
        if let Some(ring) = self.data.inventory.rings.get(id) {
            self.equipped_items.ring = Some(id.to_string());
            self.temp_stat_boosts.increase_max_mana(ring.stats.mana);
            self.temp_stat_boosts
                .give_enchantment_values(&ring.enchantments);
            return true;
        }
        false
    }

    /// Returns true if the item was unequipped.
    pub fn unequip_weapon(&mut self) -> bool {
        if let Some(id) = &self.equipped_items.weapon {
            if let Some(weapon) = self.data.inventory.weapons.get(id) {
                self.temp_stat_boosts.decrease_damage(weapon.stats.damage);
                self.temp_stat_boosts
                    .decrease_crit_hit_rate(weapon.stats.crit_hit_rate);
                self.temp_stat_boosts
                    .remove_enchantment_values(&weapon.enchantments);
                self.equipped_items.weapon = None;
                return true;
            }
        }
        false
    }

    /// Returns true if the item was unequipped.
    pub fn unequip_armor(&mut self) -> bool {
        if let Some(id) = &self.equipped_items.armor {
            if let Some(armor) = self.data.inventory.armors.get(id) {
                self.temp_stat_boosts
                    .decrease_max_health(armor.stats.health);
                self.temp_stat_boosts.decrease_defense(armor.stats.defense);
                self.temp_stat_boosts
                    .remove_enchantment_values(&armor.enchantments);
                self.equipped_items.armor = None;
                return true;
            }
        }
        false
    }

    /// Returns true if the item was unequipped.
    pub fn unequip_ring(&mut self) -> bool {
        if let Some(id) = &self.equipped_items.ring {
            if let Some(ring) = self.data.inventory.rings.get(id) {
                self.temp_stat_boosts.decrease_max_mana(ring.stats.mana);
                self.temp_stat_boosts
                    .remove_enchantment_values(&ring.enchantments);
                self.equipped_items.ring = None;
                return true;
            }
        }
        false
    }

    pub fn gain_exp(&mut self, exp: u32) {
        self.data.stats.general_stats.current_exp += exp;
        self.data.stats.general_stats.total_exp += exp;

        while self.data.stats.general_stats.current_exp
            >= self.data.stats.general_stats.required_exp
        {
            self.level_up();
        }
    }

    pub fn level_up(&mut self) -> u32 {
        let next_level = self.data.stats.general_stats.character_level + 1;
        self.data.stats.general_stats.character_level = next_level;
        self.data.stats.general_stats.current_exp -= self.data.stats.general_stats.required_exp;
        self.data.stats.general_stats.required_exp =
            (self.data.stats.general_stats.required_exp as f32 * 1.2).round() as u32;

        if next_level
            > self
                .data
                .stats
                .general_stats
                .highest_character_level_achieved
        {
            self.data
                .stats
                .general_stats
                .highest_character_level_achieved = next_level
        }

        // increase max health and damage on level up
        self.data.stats.combat_stats.max_health += 5;
        self.data.stats.combat_stats.damage += 2;

        next_level
    }

    pub fn increase_deaths(&mut self) {
        self.data.stats.general_stats.deaths += 1;
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
        self.give_weapon(&generate_random_weapon(
            WEAPON_BASE_VALUES,
            1,
            &self.data.metadata.class,
        ));
    }

    /// Returns true if deleted.
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
        if let Some(weapon) = self.data.inventory.weapons.get(id) {
            if weapon.is_equipped(&self) {
                self.unequip_weapon();
            }
        }
        if let Some(_) = self.data.inventory.weapons.remove(id) {
            return true;
        }
        false
    }

    pub fn delete_armor(&mut self, id: &str) -> bool {
        if let Some(armor) = self.data.inventory.armors.get(id) {
            if armor.is_equipped(&self) {
                self.unequip_armor();
            }
        }
        if let Some(_) = self.data.inventory.armors.remove(id) {
            return true;
        }
        false
    }

    pub fn delete_ring(&mut self, id: &str) -> bool {
        if let Some(ring) = self.data.inventory.rings.get(id) {
            if ring.is_equipped(&self) {
                self.unequip_ring();
            }
        }
        if let Some(_) = self.data.inventory.rings.remove(id) {
            return true;
        }
        false
    }

    pub fn get_total_damage(&self) -> u32 {
        self.data.stats.combat_stats.damage + self.temp_stat_boosts.damage
    }

    pub fn get_total_crit_hit_rate(&self) -> f64 {
        let total = self.data.stats.combat_stats.critical_hit_rate
            + self.temp_stat_boosts.critical_hit_rate;
        if total > 1.0 {
            return 1.0;
        }
        total
    }

    pub fn get_total_crit_damage_multiplier(&self) -> f64 {
        self.data.stats.combat_stats.critical_damage_multiplier
            + self.temp_stat_boosts.critical_damage_multiplier
    }

    pub fn get_total_health(&self) -> u32 {
        self.data.stats.combat_stats.max_health + self.temp_stat_boosts.max_health
    }

    pub fn get_total_defense(&self) -> u32 {
        self.data.stats.combat_stats.defense + self.temp_stat_boosts.defense
    }

    pub fn get_total_mana(&self) -> u32 {
        self.data.stats.combat_stats.max_mana + self.temp_stat_boosts.max_mana
    }

    pub fn get_crit_hit_damage(&self) -> u32 {
        (self.get_total_damage() as f64 * self.get_total_crit_damage_multiplier()) as u32
    }

    /// Returns the amount of damage taken.
    pub fn take_damage(&mut self, damage: u32) -> u32 {
        let reduced_damage = self.get_reduced_damage_taken(damage);
        if reduced_damage >= self.temp_stats.current_health {
            self.temp_stats.current_health = 0;
        } else {
            self.temp_stats.current_health -= reduced_damage;
        }
        reduced_damage
    }

    /// Neglects the player's defense. Returns the amount of damage taken.
    pub fn take_pure_damage(&mut self, damage: u32) -> u32 {
        if damage >= self.temp_stats.current_health {
            self.temp_stats.current_health = 0;
        } else {
            self.temp_stats.current_health -= damage;
        }
        damage
    }

    /// Returns the amount of damage to take after defense reduction.
    fn get_reduced_damage_taken(&self, damage: u32) -> u32 {
        if self.get_total_defense() >= damage {
            return 0;
        }
        damage - self.get_total_defense()
    }

    /// Returns the amount of restored health.
    pub fn restore_health(&mut self, amount: u32) -> u32 {
        let max_health = self.get_total_health();
        if self.temp_stats.current_health + amount >= max_health {
            let current_health = self.temp_stats.current_health;
            self.temp_stats.current_health = max_health;
            return max_health - current_health;
        }
        self.temp_stats.current_health += amount;
        amount
    }

    /// Returns the amount of restored mana.
    pub fn restore_mana(&mut self, amount: u32) -> u32 {
        let max_mana = self.get_total_mana();
        if self.temp_stats.current_mana + amount >= max_mana {
            let current_mana = self.temp_stats.current_mana;
            self.temp_stats.current_mana = max_mana;
            return max_mana - current_mana;
        }
        self.temp_stats.current_mana += amount;
        amount
    }

    pub fn is_dead(&self) -> bool {
        self.temp_stats.current_health == 0
    }

    /// Returns enemy fight text.
    pub fn attack_enemy(&self, enemy: &mut Enemy) -> String {
        if is_critical_hit(self.get_total_crit_hit_rate()) {
            let damage_taken = enemy.take_damage(self.get_crit_hit_damage());
            return format!(
                "Player attacked! Enemy took {} damage (Critical Hit)",
                damage_taken
            );
        }
        let damage_taken = enemy.take_damage(self.get_total_damage());
        return format!("Player attacked! Enemy took {} damage", damage_taken);
    }

    pub fn dungeon_floor_completed(&mut self, next_floor: u32) {
        self.data.stats.general_stats.current_dungeon_floor = next_floor;
        if next_floor > self.data.stats.general_stats.highest_dungeon_floor_achieved {
            self.data.stats.general_stats.highest_dungeon_floor_achieved = next_floor;
        }
    }

    pub fn reset_character_on_death(&mut self) {
        self.unequip_weapon();
        self.unequip_armor();
        self.unequip_ring();
        self.equipped_items.weapon = None;
        self.equipped_items.armor = None;
        self.equipped_items.ring = None;
        self.data.inventory.clear_consumables();
        self.data.inventory.clear_weapons();
        self.data.inventory.clear_armors();
        self.data.inventory.clear_rings();
        self.data.currency.gold = 0;
        self.data.stats.general_stats.character_level = 1;
        self.data.stats.general_stats.current_dungeon_floor = 1;
        self.data.stats.general_stats.current_exp = 0;
        self.data.stats.general_stats.required_exp = STARTING_REQUIRED_EXP;
        self.data.stats.general_stats.total_exp = 0;
        match self.data.metadata.class {
            CharacterClass::Mage => self.data.stats.combat_stats = CLASS_MAGE_STARTING_STATS,
            CharacterClass::Cleric => self.data.stats.combat_stats = CLASS_CLERIC_STARTING_STATS,
            CharacterClass::Assassin => {
                self.data.stats.combat_stats = CLASS_ASSASSIN_STARTING_STATS
            }
            CharacterClass::Warrior => self.data.stats.combat_stats = CLASS_WARRIOR_STARTING_STATS,
            CharacterClass::Knight => self.data.stats.combat_stats = CLASS_KNIGHT_STARTING_STATS,
        }
        self.temp_stats.current_health = self.data.stats.combat_stats.max_health;
        self.temp_stats.current_mana = self.data.stats.combat_stats.max_mana;
        self.temp_stat_boosts.reset();

        let weapon = create_starter_weapon(&self.data.metadata.class);
        self.give_weapon(&weapon);
        self.equip_weapon(&weapon.id);
    }

    /// Returns enemy fight text.
    pub fn use_skill(&mut self, enemy: &mut Enemy) -> String {
        let skill = get_character_skill(&self.data.metadata.class);
        match skill {
            CharacterSkill::MagicProjectile => {
                let damage = (enemy.stats.max_health as f64 * 0.20) as u32;
                let damage_taken = enemy.take_pure_damage(damage);
                return format!(
                    "Player used skill {}! Enemy took {} damage",
                    &skill, damage_taken
                );
            }
            CharacterSkill::Recover => {
                let heal_percentage = 0.45;
                let restored_health =
                    self.restore_health((heal_percentage * self.get_total_health() as f64) as u32);
                return format!(
                    "Player used skill {}! Player restored {} health points",
                    &skill, restored_health
                );
            }
            CharacterSkill::Stealth => {
                self.temp_stat_boosts.increase_crit_damage_multiplier(0.4);
                return format!(
                    "Player used skill {}! Player's critical damage multiplier increased by 0.4",
                    &skill
                );
            }
            CharacterSkill::BattleCry => {
                let increased_damage = (0.3 * self.get_total_damage() as f64) as u32;
                self.temp_stat_boosts.increase_damage(increased_damage);
                return format!(
                    "Player used skill {}! Player's damage increased by {}",
                    &skill, increased_damage
                );
            }
            CharacterSkill::ArmorUp => {
                let increased_defense = self.data.stats.general_stats.character_level;
                self.temp_stat_boosts.increase_defense(increased_defense);
                return format!(
                    "Player used skill {}! Player's defense increased by {}",
                    &skill, increased_defense
                );
            }
        }
    }
}

#[derive(Clone)]
pub struct TemporaryStats {
    pub current_health: u32,
    pub current_mana: u32,
}

/// Session only equipped items. Not saved to game data.
/// Game data tracks which items are equipped so the game can
/// correctly equip the correct items when loading a player character.
#[derive(Clone)]
pub struct EquippedItems {
    /// ID of the item.
    pub weapon: Option<String>,
    /// ID of the item.
    pub armor: Option<String>,
    /// ID of the item.
    pub ring: Option<String>,
}

#[derive(Clone)]
pub struct TemporaryStatBoosts {
    pub max_health: u32,
    pub max_mana: u32,
    pub defense: u32,
    pub damage: u32,
    pub critical_damage_multiplier: f64,
    pub critical_hit_rate: f64,
}

impl TemporaryStatBoosts {
    pub fn reset(&mut self) {
        self.max_health = 0;
        self.max_mana = 0;
        self.defense = 0;
        self.damage = 0;
        self.critical_damage_multiplier = 0.0;
        self.critical_hit_rate = 0.0;
    }

    pub fn increase_damage(&mut self, amount: u32) {
        self.damage += amount;
    }

    pub fn decrease_damage(&mut self, amount: u32) {
        if amount > self.damage {
            return self.damage = 0;
        }
        self.damage -= amount;
    }

    pub fn increase_crit_hit_rate(&mut self, amount: f64) {
        self.critical_hit_rate += amount;
    }

    pub fn decrease_crit_hit_rate(&mut self, amount: f64) {
        if amount > self.critical_hit_rate {
            return self.critical_hit_rate = 0.0;
        }
        self.critical_hit_rate -= amount;
    }

    pub fn increase_crit_damage_multiplier(&mut self, amount: f64) {
        self.critical_damage_multiplier += amount;
    }

    pub fn decrease_crit_damage_multiplier(&mut self, amount: f64) {
        if amount > self.critical_damage_multiplier {
            return self.critical_damage_multiplier = 0.0;
        }
        self.critical_damage_multiplier -= amount;
    }

    pub fn increase_max_health(&mut self, amount: u32) {
        self.max_health += amount;
    }

    pub fn decrease_max_health(&mut self, amount: u32) {
        if amount > self.max_health {
            return self.max_health = 0;
        }
        self.max_health -= amount;
    }

    pub fn increase_defense(&mut self, amount: u32) {
        self.defense += amount;
    }

    pub fn decrease_defense(&mut self, amount: u32) {
        if amount > self.defense {
            return self.defense = 0;
        }
        self.defense -= amount;
    }

    pub fn increase_max_mana(&mut self, amount: u32) {
        self.max_mana += amount;
    }

    pub fn decrease_max_mana(&mut self, amount: u32) {
        if amount > self.max_mana {
            return self.max_mana = 0;
        }
        self.max_mana -= amount;
    }

    pub fn give_enchantment_values(&mut self, enchantments: &Vec<Enchantment>) {
        for enchantment in enchantments {
            match enchantment {
                Enchantment::Damage(val) => self.increase_damage(*val),
                Enchantment::CritHitRate(val) => self.increase_crit_hit_rate(*val),
                Enchantment::Health(val) => self.increase_max_health(*val),
                Enchantment::Defense(val) => self.increase_defense(*val),
                Enchantment::Mana(val) => self.increase_max_mana(*val),
                _ => {}
            }
        }
    }

    fn remove_enchantment_values(&mut self, enchantments: &Vec<Enchantment>) {
        for enchantment in enchantments {
            match enchantment {
                Enchantment::Damage(val) => self.decrease_damage(*val),
                Enchantment::CritHitRate(val) => self.decrease_crit_hit_rate(*val),
                Enchantment::Health(val) => self.decrease_max_health(*val),
                Enchantment::Defense(val) => self.decrease_defense(*val),
                Enchantment::Mana(val) => self.decrease_max_mana(*val),
                _ => {}
            }
        }
    }
}
