use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number =rand::thread_rng().gen_range(1..=100);
    println!("The secret number is:{secret_number}");
    println!("Please input your guess.");
    let mut guess =String::new();
    io::stdin().
    read_line(&mut guess)
    .expect("Failed to read line");
    println!("You guessed: {guess}");
    let x=5;
    let y=10;
    println!("x is {x},y is {y}");

}
