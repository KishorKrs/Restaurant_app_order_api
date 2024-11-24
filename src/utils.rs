use rand::Rng;

// Generate a random cook time between 5 and 15
pub fn generate_random_cook_time() -> i32 {
    rand::thread_rng().gen_range(5..=15)
}
