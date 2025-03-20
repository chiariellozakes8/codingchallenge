fn main() {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1..=6);
    println!("The roll was {}", roll);
}
