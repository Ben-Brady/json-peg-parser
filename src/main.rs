mod scanner;
mod tokenizer;
mod parser;
mod error;
pub use error::Error;
pub use tokenizer::{Token, Tokenizer};
pub use parser::{parse, JSON};

#[cfg(test)]
mod tests;

fn main() {
    use std::process;
    use std::fs;
    use std::env;

    let path: String = env::args().nth(1).unwrap();
    let json = fs::read_to_string(path).unwrap();

    match parse(&json) {
        Ok(_) => { process::exit(0) }
        Err(_) => { process::exit(1) }
    }
}