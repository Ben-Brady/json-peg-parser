#[allow(dead_code, unused_variables)]

mod scanner;
pub mod tokenizer;
mod parser;
pub use tokenizer::{Token, Tokenizer};
pub use parser::{parse, JSON};
mod tests;

