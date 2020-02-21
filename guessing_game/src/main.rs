use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 10!\nPlease input your guess.");

    let secret_number = rand::thread_rng().gen_range(1,11);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    println!("The secret number is: {}", secret_number)
}
