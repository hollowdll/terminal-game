use rand::Rng;

pub fn is_critical_hit(rate: f64) -> bool {
    let mut rng = rand::thread_rng();
    let rand_rate = rng.gen_range(0.0..1.0);
    if rand_rate < rate {
        return true;
    }
    false
}
