use rand::Rng;

pub fn pin(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..=9).to_string())
        .collect()
}
