fn main() {
    println!("Hello, world!");
    let mut a = String::from("Samrat");
    a.push_str(" K S");
    println!("{}", a);
    let first = &a[..6];
    println!("First:{}", first);
    // a[1] = 's'; Caanot be changed by index


    let s = String::from("test owner");
    // let mut s1 = &s; // This means we are borrowing, we cannot barrow as mutable variable
    let mut s1 = s;
    s1.push_str("and Test append");
    println!("{}", s1);
}