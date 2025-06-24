use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number =rand::thread_rng().random_range(1..=100);
    println!("The secret number is:{secret_number}");
    println!("Please input your guess.");
    let mut guess =String::new();
    io::stdin().
    read_line(&mut guess)
    .expect("Failed to read line");
    let guess:u32 =guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Equal=>println!("you WIN!"),
        Ordering::Greater=>println!("Too big!"),
        Ordering::Less=>println!("too small"),
    }

}


fn main() {
   another_function(10);
}

fn another_function(x:u32){
    println!("the number is {x}");
}
