use crate::util::is_chance_success;

pub fn is_critical_hit(rate: f64) -> bool {
    is_chance_success(rate)
}
