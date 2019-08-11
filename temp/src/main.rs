//use std::env;  
//
fn main() {     
    // let mut argv = env::args();
    // let arg: String = argv.nth(1).unwrap(); // error 1
    // let n: i32 = arg.parse().unwrap(); // error 2
    // println!("{}", 2 * n);
    let s = "Samrat K S";
    println!("Find S in s {:#?}", find(&s, 'r')); 
}


fn find(haystack: &str, needle: char) -> Option<usize> {     
    for (offset, c) in haystack.char_indices() {         
        if c == needle {             
            return Some(offset);         
        }     
    }     
    None 
}
