extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to Guess the Secret Number Game!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);

    loop 
    {
        println!("Please enter your guess.");


        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

        // have to cast the guess into an int
        // Rust allows us to shadow the previous value of guess
        // with a new one. The : tells Rust we'll annotate the vars type
        let guess: u32 = match guess.trim().parse() 
                        {
                        Ok(num) => num,
                        // ask for another guess if not a number.
                        // The _ is a catchall value
                        Err(_) => continue, 
                        };

        println!("You guessed: {}", guess);

        // a match expression is made of of arms. An arm
        // consists of a pattern.
        match guess.cmp(&secret_number) 
        {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => 
            {
                println!("you win!!!");
                break;
            }
        }
    }
}
