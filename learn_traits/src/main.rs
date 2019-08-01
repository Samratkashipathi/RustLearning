use std::fmt::Display;

#[derive(Debug)]
struct Point <T>{
    x : T,
    y : T
}

impl<T> Point <T>{
    fn new (x : T, y: T) -> Self {
        Self {x: x, y: y}
    }
}

impl <T> Point <T> 
    where T : PartialOrd + Display
{
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("x is greater than y")
        }
        else{
            println!("y is greater than x")
        }
    }
}
fn main() {
    println!("Hello, world!");

    let p = Point::new(3, 4);

    println!("{:#?}", p)
}
