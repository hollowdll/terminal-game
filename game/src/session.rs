pub struct SessionData {
    pub temp_stats: TemporaryStats,
    pub temp_stat_boosts: TemporaryStatBoosts,
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
