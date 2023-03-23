use crate::peg::{JSONParser,Rule};
use std::collections::HashMap;
use pest::{iterators::Pair, Parser};

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    JSONParsingFailure
}

#[derive(Debug,  PartialEq)]
pub enum Value {
    Null,
    String(String),
    Boolean(bool),
    Number(f64),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

pub fn parse(json: &str) -> Result<Value, Error> {
    let tokens = JSONParser::parse(Rule::value, json).unwrap();
    let value = tokens.into_iter().next().unwrap();
    parse_pair(value)
}

fn parse_pair(value: Pair<Rule>) -> Result<Value, Error> {
    match value.as_rule() {
        Rule::js_false => Ok(Value::Boolean(false)),
        Rule::js_true => Ok(Value::Boolean(true)),
        Rule::js_null => Ok(Value::Null),
        Rule::js_number => Ok(Value::Number(parse_number(value)?)),
        Rule::js_string => Ok(Value::String(parse_string(value)?)),
        Rule::js_array => Ok(Value::Array(parse_array(value)?)),
        Rule::js_object => Ok(Value::Object(parse_object(value)?)),
        _ => Err(Error::JSONParsingFailure)
    }
}

fn parse_number(value: Pair<Rule>) -> Result<f64, Error> {
    let num_string = value.as_str();
    let num = num_string.parse::<f64>().or( Err(Error::JSONParsingFailure))?;
    Ok(num)
}

fn parse_string(value: Pair<Rule>) -> Result<String, Error> {
    let inner_string = value.into_inner().next().ok_or(Error::JSONParsingFailure)?;
    if !(matches!(inner_string.as_rule(), Rule::inner_string)) {
        return Err(Error::JSONParsingFailure);
    }
    
    let string = inner_string.as_str();

    fn str_from_codepoint(codepoint: &str) -> String {
        let codepoint = u64::from_str_radix(codepoint, 16).unwrap();
        char::from_u32(codepoint as u32).unwrap().to_string()
    }

    let string = string
        .replace("\\\"", "\"")
        .replace("\\/", "/")
        .replace("\\\\", "\\")
        .replace("\\b", &str_from_codepoint("08"))
        .replace("\\f", &str_from_codepoint("0C"))
        .replace("\\n", &str_from_codepoint("0A"))
        .replace("\\r", &str_from_codepoint("0D"))
        .replace("\\n", &str_from_codepoint("09"));
    Ok(string)
}

fn parse_array(pair: Pair<Rule>) -> Result<Vec<Value>, Error> {
    let mut values = Vec::new();
    for pair in pair.into_inner() {
        let value = parse_pair(pair)?;
        values.push(value)
    };

    Ok(values)
}

#[allow(clippy::iter_nth_zero)]
fn parse_object(pair: Pair<Rule>) -> Result<HashMap<String, Value>, Error> {
    let mut object = HashMap::new();
    for pair in pair.into_inner() {
        let mut key_value_pair = pair.into_inner();
        let key = key_value_pair.next().ok_or(Error::JSONParsingFailure)?;
        let value = key_value_pair.next().ok_or(Error::JSONParsingFailure)?;
        let key = parse_string(key)?;
        let value = parse_pair(value)?;
        object.insert(key, value);
    }

    Ok(object)
}
