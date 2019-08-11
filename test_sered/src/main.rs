extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::str;

#[derive(Deserialize, Serialize, Debug)]
struct Move {
    up: i32,
    down: i32,
    right: i32,
    left: i32,
}

fn main() {
    let mv = Move {
        up: 2,
        down: 10,
        right: 10,
        left: 1,
    };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&mv).unwrap();

    println!("Deserialized version : {:#?}", mv);

    println!("Serialized Move struct : {}", serialized);

    // let mut file = File::create("temp.txt").unwrap();
    // file.write_all(serialized.as_bytes()).unwrap();

    // let mut content = String::new();
    // let mut json_file = File::open("temp.txt").unwrap();
    // json_file.read_to_string(&mut content).unwrap();
    // println!("Json file content : {:#?}", content);

    // let mv1: Move = serde_json::from_str(&content).unwrap();
    // println!("Dese data from file : {:#?}", mv1)
    let vec_content = serialized.as_bytes();
    println!("Vec of string bytes : {:#?}", vec_content);
    println!(
        "String from vector : {}",
        str::from_utf8(&vec_content).unwrap()
    );
}
