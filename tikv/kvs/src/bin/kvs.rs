#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("get") {
        let get_param = matches.value_of(String::from("get")).unwrap();
        get_method(get_param);
    }

    if matches.is_present("set") {
        println!("Set");
    }

    if matches.is_present("rm") {
        println!("Remove");
    }
}

fn get_method(key : &str) {
    println!("Key : {}", key)
}