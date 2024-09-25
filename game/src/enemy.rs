use rand::{thread_rng, Rng};

use crate::{fight::is_critical_hit, session::PlayerCharacter};

pub const EXP_MULTIPLIER_NORMAL_ENEMY: u32 = 1;
pub const EXP_MULTIPLIER_BOSS_ENEMY: u32 = 3;
pub const EXP_MULTIPLIER_ANCIENT_ENEMY: u32 = 4;
pub const GOLD_MULTIPLIER_NORMAL_ENEMY: u32 = 1;
pub const GOLD_MULTIPLIER_BOSS_ENEMY: u32 = 3;
pub const GOLD_MULTIPLIER_ANCIENT_ENEMY: u32 = 4;
pub const ENEMY_SKILL_CHANCE: f64 = 0.35;
pub const ENEMY_CRIT_HIT_RATE: f64 = 0.20;
pub const ENEMY_CRIT_DAMAGE_MULTIPLIER: f64 = 2.0;

pub const NORMAL_ENEMY_NAME_SKELETON: &str = "Skeleton";
pub const NORMAL_ENEMY_NAME_GOBLIN: &str = "Goblin";
pub const NORMAL_ENEMY_NAME_OGRE: &str = "Ogre";
pub const NORMAL_ENEMY_NAME_GOLEM: &str = "Golem";
pub const NORMAL_ENEMY_NAMES: [&str; 4] = [
    NORMAL_ENEMY_NAME_SKELETON,
    NORMAL_ENEMY_NAME_GOBLIN,
    NORMAL_ENEMY_NAME_OGRE,
    NORMAL_ENEMY_NAME_GOLEM,
];

pub const BOSS_ENEMY_NAME_OGRE_KING: &str = "Ogre King";
pub const BOSS_ENEMY_NAME_FIRE_DRAGON: &str = "Fire Dragon";
pub const BOSS_ENEMY_NAME_UNDEAD_SORCERER: &str = "Undead Sorcerer";
pub const BOSS_ENEMY_NAMES: [&str; 3] = [
    BOSS_ENEMY_NAME_OGRE_KING,
    BOSS_ENEMY_NAME_FIRE_DRAGON,
    BOSS_ENEMY_NAME_UNDEAD_SORCERER,
];

pub const ANCIENT_ENEMY_NAME_KNIGHT: &str = "Lancelot, the Divine Knight";
pub const ANCIENT_ENEMY_NAMES: [&str; 1] = [ANCIENT_ENEMY_NAME_KNIGHT];

pub const LESSER_ENEMY_BASE_STATS: EnemyBaseStats = EnemyBaseStats {
    health: 40,
    defense: 0,
    damage: 7,
};
pub const GREATER_ENEMY_BASE_STATS: EnemyBaseStats = EnemyBaseStats {
    health: 50,
    defense: 1,
    damage: 9,
};
pub const BOSS_ENEMY_BASE_STATS: EnemyBaseStats = EnemyBaseStats {
    health: 100,
    defense: 2,
    damage: 14,
};
pub const ANCIENT_ENEMY_BASE_STATS: EnemyBaseStats = EnemyBaseStats {
    health: 250,
    defense: 3,
    damage: 16,
};

#[derive(Debug, Clone)]
pub enum EnemySkill {
    Smash,
    FireBreath,
    StatusAilment,
    DivineBlessing,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: &'static str,
    pub kind: EnemyKind,
    pub level: u32,
    pub stats: EnemyStats,
    pub stat_boosts: EnemyStatBoosts,
    pub skill: Option<EnemySkill>,
}

pub struct EnemyBaseStats {
    pub health: u32,
    pub defense: u32,
    pub damage: u32,
}

impl Enemy {
    pub fn new_normal(dungeon_floor: u32, name: &'static str, base_stats: &EnemyBaseStats) -> Self {
        Self {
            name,
            kind: EnemyKind::Normal,
            level: dungeon_floor,
            stats: EnemyStats {
                max_health: base_stats.health + (40 * dungeon_floor),
                current_health: base_stats.health + (40 * dungeon_floor),
                defense: base_stats.defense + (1 * dungeon_floor),
                damage: base_stats.damage + (5 * dungeon_floor),
                crit_hit_rate: ENEMY_CRIT_HIT_RATE,
                crit_damage_multiplier: ENEMY_CRIT_DAMAGE_MULTIPLIER,
            },
            stat_boosts: EnemyStatBoosts {
                defense: 0,
                damage: 0,
            },
            skill: None,
        }
    }

    pub fn new_boss(dungeon_floor: u32, name: &'static str, base_stats: &EnemyBaseStats) -> Self {
        let skill = match name {
            BOSS_ENEMY_NAME_OGRE_KING => EnemySkill::Smash,
            BOSS_ENEMY_NAME_FIRE_DRAGON => EnemySkill::FireBreath,
            BOSS_ENEMY_NAME_UNDEAD_SORCERER => EnemySkill::StatusAilment,
            _ => EnemySkill::Unknown,
        };
        Self {
            name,
            kind: EnemyKind::Boss,
            level: dungeon_floor,
            stats: EnemyStats {
                max_health: base_stats.health + (60 * dungeon_floor),
                current_health: base_stats.health + (60 * dungeon_floor),
                defense: base_stats.defense + (2 * dungeon_floor),
                damage: base_stats.damage + (6 * dungeon_floor),
                crit_hit_rate: ENEMY_CRIT_HIT_RATE,
                crit_damage_multiplier: ENEMY_CRIT_DAMAGE_MULTIPLIER,
            },
            stat_boosts: EnemyStatBoosts {
                defense: 0,
                damage: 0,
            },
            skill: Some(skill),
        }
    }

    pub fn new_ancient(level: u32, name: &'static str, base_stats: &EnemyBaseStats) -> Self {
        let skill = match name {
            ANCIENT_ENEMY_NAME_KNIGHT => EnemySkill::DivineBlessing,
            _ => EnemySkill::Unknown,
        };
        Self {
            name,
            kind: EnemyKind::Ancient,
            level,
            stats: EnemyStats {
                max_health: base_stats.health + (85 * level),
                current_health: base_stats.health + (85 * level),
                defense: base_stats.defense + (3 * level),
                damage: base_stats.damage + (8 * level),
                crit_hit_rate: ENEMY_CRIT_HIT_RATE,
                crit_damage_multiplier: ENEMY_CRIT_DAMAGE_MULTIPLIER,
            },
            stat_boosts: EnemyStatBoosts {
                defense: 0,
                damage: 0,
            },
            skill: Some(skill),
        }
    }

    pub fn get_display_name(&self) -> String {
        match self.kind {
            EnemyKind::Ancient => format!("{} [Ancient Boss] (Level {})", self.name, self.level),
            EnemyKind::Normal => format!("{} (Level {})", self.name, self.level),
            EnemyKind::Boss => format!("{} [Boss] (Level {})", self.name, self.level),
        }
    }

    pub fn get_total_damage(&self) -> u32 {
        self.stats.damage + self.stat_boosts.damage
    }

    pub fn get_total_defense(&self) -> u32 {
        self.stats.defense + self.stat_boosts.defense
    }

    pub fn get_total_crit_hit_rate(&self) -> f64 {
        self.stats.crit_hit_rate
    }

    pub fn get_crit_hit_damage(&self) -> u32 {
        (self.get_total_damage() as f64 * self.stats.crit_damage_multiplier) as u32
    }

    /// Returns the amount of damage taken.
    pub fn take_damage(&mut self, damage: u32) -> u32 {
        let reduced_damage = self.get_reduced_damage_taken(damage);
        if reduced_damage >= self.stats.current_health {
            self.stats.current_health = 0;
        } else {
            self.stats.current_health -= reduced_damage;
        }
        return reduced_damage;
    }

    /// Neglects the enemy's defense. Returns the amount of damage taken.
    pub fn take_pure_damage(&mut self, damage: u32) -> u32 {
        if damage >= self.stats.current_health {
            self.stats.current_health = 0;
        } else {
            self.stats.current_health -= damage;
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
        if self.stats.current_health + amount >= self.stats.max_health {
            let current_health = self.stats.current_health;
            self.stats.current_health = self.stats.max_health;
            return self.stats.max_health - current_health;
        }
        self.stats.current_health += amount;
        amount
    }

    pub fn increase_damage(&mut self, amount: u32) {
        self.stat_boosts.damage += amount;
    }

    pub fn decrease_damage(&mut self, amount: u32) {
        if amount > self.stat_boosts.damage {
            return self.stat_boosts.damage = 0;
        }
        self.stat_boosts.damage -= amount
    }

    pub fn increase_defense(&mut self, amount: u32) {
        self.stat_boosts.defense += amount;
    }

    pub fn decrease_defense(&mut self, amount: u32) {
        if amount > self.stat_boosts.defense {
            return self.stat_boosts.defense = 0;
        }
        self.stat_boosts.defense -= amount
    }

    pub fn is_dead(&self) -> bool {
        self.stats.current_health == 0
    }

    /// Returns enemy fight text.
    pub fn attack_player(&self, character: &mut PlayerCharacter) -> (&str, String) {
        if is_critical_hit(self.get_total_crit_hit_rate()) {
            let damage_taken = character.take_damage(self.get_crit_hit_damage());
            return (
                "Enemy attacked!",
                format!("Player took {} damage (Critical Hit)", damage_taken),
            );
        }
        let damage_taken = character.take_damage(self.get_total_damage());
        return (
            "Enemy attacked!",
            format!("Player took {} damage", damage_taken),
        );
    }

    pub fn use_skill(&mut self, character: &mut PlayerCharacter) -> (&str, String) {
        if let Some(skill) = &self.skill {
            match skill {
                EnemySkill::Smash => {
                    let damage = (character.get_total_health() as f64 * 0.20) as u32;
                    let damage_taken = character.take_pure_damage(damage);
                    return (
                        "Enemy used skill Smash!",
                        format!("Player took {} damage", damage_taken),
                    );
                }
                EnemySkill::FireBreath => {
                    let damage = (character.get_total_health() as f64 * 0.12) as u32;
                    let damage_taken = character.take_pure_damage(damage);
                    let reduced_defense = 2 * self.level;
                    character.temp_stat_boosts.decrease_defense(reduced_defense);
                    return (
                        "Enemy used skill Fire Breath!",
                        format!(
                            "Player took {} damage. Player's defense was reduced by {}",
                            damage_taken, reduced_defense
                        ),
                    );
                }
                EnemySkill::StatusAilment => {
                    let reduced_damage = self.level;
                    let reduced_mana = 15;
                    character.temp_stat_boosts.decrease_damage(reduced_damage);
                    character.consume_mana(reduced_mana);
                    return (
                        "Enemy used skill Status Ailment!",
                        format!(
                            "Player's damage was reduced by {}. Player's mana was reduced by {}",
                            reduced_damage, reduced_mana
                        ),
                    );
                }
                EnemySkill::DivineBlessing => {
                    let restored_health =
                        self.restore_health((0.05 * self.stats.max_health as f64) as u32);
                    let increased_damage = self.level / 2;
                    self.increase_damage(increased_damage);
                    return (
                        "Enemy used skill Divine Blessing!",
                        format!(
                            "Enemy restored {} health points. Enemy's damage was increased by {}",
                            restored_health, increased_damage
                        ),
                    );
                }
                _ => {
                    return (
                        "Enemy tried to use an unknown skill",
                        "Nothing happened".to_string(),
                    )
                }
            }
        }
        return (
            "Enemy tried to use skill",
            "But it doesn't have one".to_string(),
        );
    }
}

#[derive(Debug, Clone)]
pub struct EnemyStats {
    pub max_health: u32,
    pub current_health: u32,
    pub defense: u32,
    pub damage: u32,
    pub crit_hit_rate: f64,
    pub crit_damage_multiplier: f64,
}

#[derive(Debug, Clone)]
pub struct EnemyStatBoosts {
    pub defense: u32,
    pub damage: u32,
}

#[derive(Debug, Clone)]
pub enum EnemyKind {
    Ancient,
    Boss,
    Normal,
}

pub fn generate_random_normal_enemy(dungeon_floor: u32) -> Enemy {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..NORMAL_ENEMY_NAMES.len());
    let name = NORMAL_ENEMY_NAMES[index];
    let base_stats = match name {
        NORMAL_ENEMY_NAME_SKELETON => &LESSER_ENEMY_BASE_STATS,
        NORMAL_ENEMY_NAME_GOBLIN => &LESSER_ENEMY_BASE_STATS,
        NORMAL_ENEMY_NAME_OGRE => &GREATER_ENEMY_BASE_STATS,
        NORMAL_ENEMY_NAME_GOLEM => &GREATER_ENEMY_BASE_STATS,
        _ => &LESSER_ENEMY_BASE_STATS,
    };
    Enemy::new_normal(dungeon_floor, name, base_stats)
}

pub fn generate_random_boss_enemy(dungeon_floor: u32) -> Enemy {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..BOSS_ENEMY_NAMES.len());
    let name = BOSS_ENEMY_NAMES[index];
    Enemy::new_boss(dungeon_floor, name, &BOSS_ENEMY_BASE_STATS)
}

pub fn generate_random_ancient_enemy(level: u32) -> Enemy {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..ANCIENT_ENEMY_NAMES.len());
    let name = ANCIENT_ENEMY_NAMES[index];
    Enemy::new_ancient(level, name, &ANCIENT_ENEMY_BASE_STATS)
}
