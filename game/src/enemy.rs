pub const EXP_MULTIPLIER_NORMAL_ENEMY: u32 = 1;
pub const EXP_MULTIPLIER_BOSS_ENEMY: u32 = 3;
pub const GOLD_MULTIPLIER_NORMAL_ENEMY: u32 = 1;
pub const GOLD_MULTIPLIER_BOSS_ENEMY: u32 = 3;

pub const NORMAL_ENEMY_NAMES: NormalEnemyNames = NormalEnemyNames {
    skeleton_warrior: "Skeleton Warrior",
    skeleton_archer: "Skeleton Archer",
    skeleton_mage: "Skeleton Mage",
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
    pub skeleton_warrior: &'static str,
    pub skeleton_archer: &'static str,
    pub skeleton_mage: &'static str,
}

pub struct BossEnemyNames {
    pub ogre_king: &'static str,
    pub fire_dragon: &'static str,
}

pub struct Enemy {
    pub name: &'static str,
    pub kind: EnemyKind,
    pub stats: EnemyStats,
    pub skill: Option<EnemySkill>,
}

impl Enemy {
    pub fn new_normal(dungeon_floor: u32, name: &'static str) -> Self {
        Self {
            name,
            kind: EnemyKind::Normal,
            stats: EnemyStats {
                max_health: 50 + (25 * dungeon_floor),
                health: 50 + (25 * dungeon_floor),
                defense: 0,
                damage: 7 + (5 * dungeon_floor),
            },
            skill: None,
        }
    }

    pub fn new_boss(dungeon_floor: u32, name: &'static str, skill: EnemySkill) -> Self {
        Self {
            name,
            kind: EnemyKind::Boss,
            stats: EnemyStats {
                max_health: 150 + (50 * dungeon_floor),
                health: 150 + (50 * dungeon_floor),
                defense: 1 + (1 * dungeon_floor),
                damage: 15 + (6 * dungeon_floor),
            },
            skill: Some(skill),
        }
    }
}

pub struct EnemyStats {
    pub max_health: u32,
    pub health: u32,
    pub defense: u32,
    pub damage: u32,
}

pub enum EnemyKind {
    Boss,
    Normal,
}

pub struct EnemySkill {
    pub name: &'static str,
    pub effect: &'static str,
}
