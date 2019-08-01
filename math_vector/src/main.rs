// extern crate rand;

use std::io;

struct Guess {
    number : i32,
}

impl Guess {
    fn new(number : i32) -> Guess{
        if number < 1 || number > 100{
            panic!("Number should be b/w 1 and 100 but got {}", number)
        }

        Guess {number : number}
    }

    fn value(&self) -> i32{
        return self.value();
    }
}

fn main() {
    println!("Please enter the number:");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed to read the input number");

    let mut n : i32 = number.trim().parse().expect("Entered value should be number");

    let guess = Guess::new(n);

    println!("{}", guess.value());
}
