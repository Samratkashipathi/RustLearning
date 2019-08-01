extern crate rand;

use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    for ele in v.iter() {
        println!("{}", ele)
    }

    println!("Value at index 0 is {}", &v[0]);

    let a = match v.get(1) {
        None => {
            println!("Not found with index {}", 1);
        }
        Some(_) => {
            println!("Found with index {}", 1);
        }
    };

    println!("value of a : {:?}", a);

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    println!("{:?}", does_not_exist);

    let mut vec: Vec<i32> = Vec::new();

    for i in 1..=10 {
        vec.push(i);
    }

    println!("Vector: {:#?}", vec);
    vec = take(rand::thread_rng().gen_range(1, 10), vec);
    barrow(&vec);
    barrow(&vec);
    vec = take2(rand::thread_rng().gen_range(1, 10), vec);
    barrow1(&vec);
    println!("Vector after all take and barrow: {:#?}", vec);
    println!("End of function");
}

fn take(value: i32, mut vector: Vec<i32>) -> Vec<i32> {
    println!("Inside take function");
    vector.push(value);
    vector
}

fn take2(value: i32, mut vector: Vec<i32>) -> Vec<i32> {
    println!("Inside take 2 function");
    vector.push(value);
    vector
}

fn barrow(vector: &Vec<i32>) {
    println!("Inside barrow function");
    println!("v[10]:{}", vector[9]);
}

fn barrow1(vector: &Vec<i32>) {
    println!("Inside barrow1 function");
    println!("v[10]:{}", vector[10]);
}
