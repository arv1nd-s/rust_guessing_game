use std::io;
use std::cmp::Ordering;
use rand::Rng;

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
        
        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}