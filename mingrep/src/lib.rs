use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic!("Arguments were not provided");
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)
        .expect("Something went wrong while opening the file");

    // println!("Contents from the file : {}", content);

    println!("Matching line are :");
    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    println!("---");

    Ok(())
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

//    for line in content.lines(){
//        if line.contains(query){
//            results.push(line);
//        }
//    }

    results = content.lines().filter(|line| line.contains(query)).collect();

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_restlt() {
        let query = "duct";
        let content = "\
        Rust:\nsafe, fast, productive\nPick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, content))
    }
}