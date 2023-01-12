use crate::tokenizer::{Tokenizer, Token};
use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub enum JSON {
    Null(),
    String(String),
    Boolean(bool),
    Number(f64),
    Array(Vec<JSON>),
    Object(HashMap<String, JSON>),
}

pub fn parse(json: &str) -> Result<JSON, &'static str> {
    let mut tokenizer = Tokenizer::from_str(json);
    parse_any(tokenizer.next(), &mut tokenizer)
}

fn parse_any(initial_token: Option<Token>, tokenizer: &mut Tokenizer) -> Result<JSON, &'static str> {
    let token = match initial_token {
        Some(token) => match token {
            Token::ArrayOpen => Ok(JSON::Array(parse_array(tokenizer)?)),
            Token::ObjectOpen => Ok(JSON::Object(parse_object(tokenizer)?)),
            Token::Number(value) => Ok(JSON::Number(parse_number(value)?)),
            Token::String(value) => Ok(JSON::String(parse_string(value)?)),
            Token::Null => Ok(JSON::Null()),
            Token::True => Ok(JSON::Boolean(true)),
            Token::False => Ok(JSON::Boolean(false)),
            Token::Colon => Err("Unexpected Token: Colon"),
            Token::Comma => Err("Unexpected Token: Comma"),
            Token::ArrayClose => Err("Unexpected Token: ArrayClose"),
            Token::ObjectClose => Err("Unexpected Token: ObjectClose"),
        },
        None => Err("Unexpected End of JSON"),
    };
    println!("{:?}", token);
    token

}

fn get_next_token(tokenizer: &mut Tokenizer) -> Result<Token, &'static str>{
    tokenizer.next().ok_or("Unexpected End of Tokens")
}

fn parse_object(tokenizer: &mut Tokenizer) -> Result<HashMap<String, JSON>, &'static str> {
    let mut object = HashMap::new();

    loop {
        let key = match get_next_token(tokenizer)? {
            Token::String(key) => Ok(key),
            _ => Err("Expected object key to be a String")
        }?;

        match get_next_token(tokenizer)? {
            Token::Colon => Ok(()),
            _ => Err("Expected colon after object key")
        }?;

        let value = match get_next_token(tokenizer)? {
            Token::Colon |Token::Comma |Token::ArrayClose |Token::ObjectClose => {
                Err("Unexpected Token in Object")
            }
            token => parse_any(Some(token), tokenizer)
        }?;

        object.insert(key, value);

        match get_next_token(tokenizer)? {
            Token::Comma => { continue; },
            Token::ObjectClose => { break Ok(object) },
            _ => { break Err("Unexpected Token at end of Object") },
        };
    }
}

fn parse_array(tokenizer: &mut Tokenizer) -> Result<Vec<JSON>, &'static str> {
    let mut array = Vec::<JSON>::new();
    loop {
        let value = match get_next_token(tokenizer)? {
            Token::Colon |Token::Comma |Token::ArrayClose |Token::ObjectClose => {
                Err("Unexpected Token in Array")
            }
            token => parse_any(Some(token), tokenizer)
        }?;

        array.push(value);

        match get_next_token(tokenizer)? {
            Token::Comma => { continue;},
            Token::ArrayClose => { break Ok(array);},
            _ => { break Err("Unexpected Token in array"); }
        }
    }
}

fn parse_number(number: String) -> Result<f64, &'static str> {
    number.parse::<f64>().or(Err("Could not parse number"))
}

fn parse_string(string: String) -> Result<String, &'static str> {
    Ok(string)
}
