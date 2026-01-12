use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let mut guess = String::new();
    let secret_number: i32 = rand::rng().random_range(1..=100);
    loop {
        println!("Enter the number you guessed ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read the line ");
        let guess: u32 = guess.trim().parse().expect("Please type a number");
        match guess.cmp(&secret_number) {
            Ordering::Equal => println!("You Won!"),
            Ordering::Less => println!("Too Low "),
            Ordering::Greater => println!("Too High "),
        }
    }
}
