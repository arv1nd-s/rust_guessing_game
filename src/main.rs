use std::io;

fn main() {
    println!("Guess the number!");

    // thread_rng() gives us the particular random number generator
    // start..=end is inclusive on the lower and upper bounds
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // creates a new, empty string
        let mut guess = String::new();
        
        // the stdin function returns an instance of std::io::Stdin,
        // which is a type that represents a handle to the standard i/p.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
    }
}