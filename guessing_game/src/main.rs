fn main() {
    println!("Guess the number!");
    println!("Please input the guess: ");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed : {}", guess);
}
