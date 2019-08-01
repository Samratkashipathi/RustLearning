extern crate clap;

use clap::ArgMatches;

struct Get {
    key : String
}

struct Set {
    key : String,
    value : String
}

struct Remove {
    key : String
}

struct ArgsParser {
    option : String,
    values : String,
}

enum Option {
    Get,
    Set,
    Remove
}
impl ArgsParser {
    fn new(matches : ArgMatches){
        if ma
    }
}