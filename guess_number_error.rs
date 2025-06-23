use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret_number:{secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


use std::io;

fn main() {
   let a=[1,2,3,4,5];
   println!("Please enter an array index.");
   let mut index = String::new();
   io::stdin().read_line(&mut index).expect("not a number");
   let index:usize =index.trim().parse().expect("expect a number");
   let element = a[index];
   println!("The value of the element at index {index} is {element}") ;   
}

//在函数签名中，必须 声明每个参数的类型 
fn main() {
  println!("hello world");
  another_function(10);
   
}
fn another_function(x:u32){
    println!("haha learn function {x}");
}

fn main() {
   let x=five();
   println!("the number is {x}");
   let y=plus_one(10);
   println!("y is {y}");
}

fn five()->u32{
    5
} //不能有分号 移除分号，现在 5 是一个表达式，其值被隐式返回 或者 return 5;

fn plus_one(x:u32)->u32{ //这里必须要有一个返回的类型
    return x+1;
}
