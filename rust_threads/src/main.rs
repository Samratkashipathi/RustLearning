use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector value in thread 1 : {:#?}", v);
        for i in 1..10 {
            println!("Count : {} from Thread 1", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    handle.join().unwrap();


    println!("Resuming in the main thread");
    println!("Vector value in main thread : {:#?}", v); // Not possible as move into thread

    for i in 1..10 {
        println!("Count : {} from Main Thread", i);
        thread::sleep(Duration::from_millis(10));
    }
}