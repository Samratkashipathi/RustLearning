use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    //  Diffrent ways of error handling, uncomment below blocks

    // let f = File::open("test_open_file.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         println!("Error: {}",error);
    //         panic!("File not found");
    //     }
    // };

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("test_open_file.txt") {
    //             Ok(new_file) => {
    //                 println!("Created new file");
    //                 new_file
    //             }
    //             Err(e) => {
    //                 println!("Counld not create new file: {}", e);
    //                 panic!("Error while creating new file")
    //             }
    //         },
    //         some_other_error => panic!("Failed to open file: {:#?}", some_other_error),
    //     },
    // };

    // let f = File::open("hello.txt").map_err(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Tried to create file but there was a problem: {:?}", error);
    //         })
    //     } else {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     }
    // });

    // let f = File::open("hello.txt").unwrap();

    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let user_name = read_username_from_file().expect("Could not fetch User name from file");
    println!("User name: {:#?}", user_name);
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
