use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        let mut guess = String::new();
        println!("Guess a number ");
        io::stdin().read_line(&mut guess).expect("Invalid Input");
        let guess: u32 = match guess.trim().parse() {
            Ok(a) => a,
            Err(_) => {
                println!("Enter valid Input");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Too High!"),
            Ordering::Less => println!("Too Low!"),
        }
    }
}
