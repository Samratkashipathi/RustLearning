extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();


    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess : u32 = guess.trim().parse()
    .expect("Please type a number!");

    println!("You guessed: {}", guess);

    let secret_number = rand::thread_rng().gen_range(1,10);

    println!("Secret number is : {}",secret_number);

    match guess.cmp(&secret_number){
      Ordering::Less => println!("Less"),
      Ordering::Greater => println!("Greater"),
      Ordering::Equal => println!("Equal"),
    }

    let a:[u32;4] = [1,2,3,4];
    println!("{}",a[1]);

    let test:i8 = 500;
    println!("{}",test)
}
