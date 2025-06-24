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

fn main() {
   let x=10;
   if x>12 {
    println!("the number is big than x");
   }
   else if x==10 {
       println!("the number is equal to x");
   }
   else {
    println!("the number is smaller than x");
   }

fn main() {
    let mut counter=5;
    let result =loop{
        counter+=1;
        if counter>10 {
            break counter*2;
        }
    };
    println!("The result is {result}");
}  //The result is 22

fn main() {
    let a =[1,2,3,4,5,6];
    let mut index=0;
    while index<5 {
        println!("the value is {}",a[index]);
        index+=1;
    }
    let b=[10,20,30,40,50];
    for element in b {
        println!("the element is {element}")
    }
for number in (1..4).rev() {
        println!("the value is {}",number);
    }
let mut s = String::from("hello");
    s.push_str("world");
    println!("{s}") //helloworld 这两个冒号 :: 是运算符，允许将特定的 from 函数置于 String 类型的命名空间（namespace）下，而不需要使用类似 string_from 这样的名字


}

//可变引用
fn main() {
    let mut s1 = String::from("hello");
    change(&mut s1);
    print!("{s1}")
}

fn change( s: &mut String){
    s.push_str(",world");
}    
