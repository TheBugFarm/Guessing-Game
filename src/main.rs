use std::io;
fn main() {

    println!("Welcome to The Number Guessing Game ");
    println!("Guess a number between 1 and 100 ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read user input");
    println!("You guessed {guess}");
}
