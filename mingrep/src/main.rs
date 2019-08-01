extern crate mingrep;

use std::env;
use std::process;
use mingrep::Config;
use mingrep::run;


fn main(){
    let args: Vec<String> = env::args().collect();
    // println!("{:#?}", args);

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Error while parsing the arguments");
        process::exit(1)
    });

    println!("Query : {}, Filename : {}", config.query, config.filename);

    if let Err(e) = run(config){
        println!("Application error : {}", e);
        process::exit(1);
    };
}

