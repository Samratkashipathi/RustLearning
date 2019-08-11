extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Key Value Store")
        .version("0.1.0")
        .author("Samrat K S")
        .subcommand(SubCommand::with_name("get").arg(Arg::with_name("KEY").required(true)))
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("KEY").required(true))
                .arg(Arg::with_name("VALUE").required(true)),
        )
        .subcommand(SubCommand::with_name("rm").arg(Arg::with_name("KEY").required(true)))
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_matches)) => println!("Set"),
        ("get", Some(_matches)) => println!("Get"),
        ("rm", Some(_matches)) => println!("Remove"),
        _ => println!("Thank you"),
    };
}

