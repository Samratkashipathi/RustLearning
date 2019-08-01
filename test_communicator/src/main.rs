extern crate communicator;

fn main() {
    communicator::client::connect();
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    for ele in array.iter() {
        println!("{}", ele);
    }
}