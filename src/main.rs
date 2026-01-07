use std::io;
use rand::Rng;
fn main() {

    println!("Welcome to The Number Guessing Game ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess a number between 1 and 100 ");

    let mut guess = String::new();

    println!("The secret number is {secret_number}");

    io::stdin().read_line(&mut guess).expect("Failed to read user input");

    println!("You guessed {guess}");

}