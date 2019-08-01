use std::sync::{Mutex, Arc};
use std::thread;


fn main() {
//    let m = Mutex::new(10);
//
//    {
//        let mut num = m.lock().unwrap();
//        *num += 10;
//    }
//
//    println!("Value : {:#?}", m);


    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _i in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}