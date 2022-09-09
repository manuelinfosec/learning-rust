// rand was installed externally
extern crate rand;

// import Rng trait from rand
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_numer = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_numer);

    // display for prompt
    println!("Please input the guess: ");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
    //parse string to specified data type
    let guess: u32 = guess.trim()
                        .parse::<u32>()
    // compare guess with secret number and return Enums from Ordering.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
>>>>>>> 99182f9 (Comparison with `Ordering` & Parsing string to unsigned integer)
}
