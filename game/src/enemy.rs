use crate::session::PlayerCharacter;

pub const EXP_MULTIPLIER_NORMAL_ENEMY: u32 = 1;
pub const EXP_MULTIPLIER_BOSS_ENEMY: u32 = 3;
pub const GOLD_MULTIPLIER_NORMAL_ENEMY: u32 = 1;
pub const GOLD_MULTIPLIER_BOSS_ENEMY: u32 = 3;
pub const ENEMY_SKILL_CHANCE: f64 = 0.25;

pub const NORMAL_ENEMY_NAMES: NormalEnemyNames = NormalEnemyNames {
    skeleton: "Skeleton",
    goblin: "Goblin",
};

pub const BOSS_ENEMY_NAMES: BossEnemyNames = BossEnemyNames {
    ogre_king: "Ogre King",
    fire_dragon: "Fire Dragon",
};

pub const ENEMY_SKILL_SMASH: EnemySkill = EnemySkill {
    name: "Smash",
    effect: "Deals 30% of the player's maximun health as damage to the player.",
};

pub struct NormalEnemyNames {
    pub skeleton: &'static str,
    pub goblin: &'static str,
}

pub struct BossEnemyNames {
    pub ogre_king: &'static str,
    pub fire_dragon: &'static str,
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

impl Enemy {
    pub fn new_normal(dungeon_floor: u32, name: &'static str) -> Self {
        Self {
            name,
            kind: EnemyKind::Normal,
            level: dungeon_floor,
            stats: EnemyStats {
                max_health: 50 + (25 * dungeon_floor),
                current_health: 50 + (25 * dungeon_floor),
                defense: 0,
                damage: 7 + (5 * dungeon_floor),
            },
            stat_boosts: EnemyStatBoosts {
                defense: 0,
                damage: 0,
            },
            skill: None,
        }
    }

    pub fn new_boss(dungeon_floor: u32, name: &'static str, skill: EnemySkill) -> Self {
        Self {
            name,
            kind: EnemyKind::Boss,
            level: dungeon_floor,
            stats: EnemyStats {
                max_health: 150 + (50 * dungeon_floor),
                current_health: 150 + (50 * dungeon_floor),
                defense: 1 + (1 * dungeon_floor),
                damage: 15 + (6 * dungeon_floor),
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
    pub fn attack_player(&self, character: &mut PlayerCharacter) -> String {
        let damage_taken = character.take_damage(self.get_total_damage());
        return format!("Enemy attacked! Player took {} damage", damage_taken);
    }
}

#[derive(Debug, Clone)]
pub struct EnemyStats {
    pub max_health: u32,
    pub current_health: u32,
    pub defense: u32,
    pub damage: u32,
}

#[derive(Debug, Clone)]
pub struct EnemyStatBoosts {
    pub defense: u32,
    pub damage: u32,
}

#[derive(Debug, Clone)]
pub struct EnemySkill {
    pub name: &'static str,
    pub effect: &'static str,
}

#[derive(Debug, Clone)]
pub enum EnemyKind {
    Boss,
    Normal,
}
