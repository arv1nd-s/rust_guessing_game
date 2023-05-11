fn main() {
    println!("Guess the number!");

    // thread_rng() gives us the particular random number generator
    // start..=end is inclusive on the lower and upper bounds
    let secret_number = rand::thread_rng().gen_range(1..=100);
}