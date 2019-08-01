#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[derive(PartialEq, Debug)]
struct Counter {
    count: u32,
}

fn shoe_in_my_size(shoes: Vec<Shoe>, shoes_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoes_size).collect()
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 100 {
            return Some(self.count);
        }

        None
    }
}

fn main() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let shoes_10 = shoe_in_my_size(shoes, 10);
    println!("{:#?}", shoes_10);

    let mut a = Counter::new();
//    for i in 1..100 {
//        println!("{:#?}", a.next())
//    }
    for val in a {
        println!("{:#?}", val)
    }
}