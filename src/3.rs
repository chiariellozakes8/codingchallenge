
use std::time::{SystemTime, UNIX_EPOCH};
let mut rng = rand::thread_rng();
let start = SystemTime::now() - Duration::from_secs(60 * 60 * 24); // 1 day ago
let end = SystemTime::now();
let delta = end.duration_since(start).unwrap();
let mut buffer = String::new();
for _ in 0..10 {
    let random_number: u32 = rng.gen_range(1, 10);
    buffer.push_str(&random_number.to_string());
}
buffer