use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: None,
        }
    }

    fn value(&mut self, args: u32) -> u32 {
        println!("Getting value");
        match self.value {
            Some(v) => {
                println!("Cacher Hit!");
                v
            }
            None => {
                println!("Cacher Miss!");
                let v = (self.calculation)(args);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

//    let expensive_closure = |num: u32| -> u32 {
//        println!("This is a closure test");
//        thread::sleep(Duration::from_secs(2));
//        num
//    };

    let i = 20;
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

//    println!("{}", expensive_closure(10))
    println!("{}", expensive_closure.value(i));
    println!("{}", expensive_closure.value(i))
}
