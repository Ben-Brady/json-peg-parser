mod peg;
mod parser;
mod error;
pub use error::Error;
pub use parser::{parse, Value};

extern crate pest;
extern crate pest_derive;

fn main() {
    use std::fs;
    use std::env;

    let path: String = env::args().nth(1).expect("Need to provide a json file path");
    let json = fs::read_to_string(path).expect("Could not read file");
    let parsed = parse(&json);
    let parsed = parsed.unwrap();
    println!("{:#?}", parsed);
    // For json test suite
    // match parsed {
    //     Ok(_) => { process::exit(0) }
    //     Err(_) => { process::exit(1) }
    // }
}