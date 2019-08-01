use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Thread 1 count : {}", i);
            tx.send(i).unwrap();
        }
    });

    let handle2 = thread::spawn(move || {
        // Rust is throwing error if sending different type of messages for the same receiver
        // let v = vec!["Hi", "This", "is", "from", "thread", "2"];
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        for i in v {
            tx1.send(i);
        }
    });

    handle.join().unwrap();

    for received in rx {
        println!("Received : {}", received);
    }
}