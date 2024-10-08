use rand::{thread_rng, Rng};

pub const BASE_GOLD_MIN: u32 = 75;
pub const BASE_GOLD_MAX: u32 = 100;
pub const GOLD_MULTIPLIER_TREASURE_CHEST: u32 = 2;

pub fn random_gold_amount(min_gold: u32, max_gold: u32, multiplier: u32) -> u32 {
    let mut rng = thread_rng();
    let base_gold = rng.gen_range(min_gold..=max_gold);
    return base_gold * multiplier;
}
