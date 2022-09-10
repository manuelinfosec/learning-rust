// rand was installed externally
extern crate rand;

// import Rng trait from rand
use rand::Rng;

// import Ordering enum & IO from standard library
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // maximum number of guesses allowed
    const MAX_GUESSES: u32 = 5;

    // this counter tracks the number of guesses
    let mut counter: u32 = 1;

    // generate a random number with the thread_rng() trait
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    // allow for multiple retries
    let final_secret: u32 = loop {
        // update counter value
        counter += 1;

        // if the number of guesses hit the threshold
        if counter > MAX_GUESSES {
            println!("You've hit the maximum number of retries.");
            break secret_number;
        }

        // display for prompt
        println!("Please input the guess: ");

        // create a mutable, empty string variable
        let mut guess: String = String::new();

        // collect input from standard input and save to the string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        //parse string to specified data type
        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // display guess result
        println!("You guessed : {}", guess);

        // compare guess with secret number and return Enums from Ordering.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),

            // do some extras and return the secret number
            Ordering::Equal => {
                println!("You win!");
                break secret_number;
            }
        };

    };

    // reveal secret number
    println!("\nThe secret number was {final_secret}");
}