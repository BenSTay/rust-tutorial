use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 10!\nPlease input your guess.");

    let secret_number = rand::thread_rng().gen_range(1,11);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u8 = guess.trim().parse()
        .expect("Not a number!");

    println!("You guessed: {}", guess);
    println!("The secret number is {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You got it!"),
    }
}
