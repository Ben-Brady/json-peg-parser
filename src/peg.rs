use pest_derive::Parser;

#[derive(Parser)]
#[grammar="grammar/json.pest"]
pub struct JSONParser;