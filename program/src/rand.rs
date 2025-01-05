use rand::Rng;

pub fn generate_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>()
}