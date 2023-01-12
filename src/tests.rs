
#[cfg(test)]
mod tokenizer {
    use crate::{Tokenizer, Token};

    #[test]
    fn parses_null() {
        let json = "
        {
            \"true\": true,
            \"false\": false,
        }";
        let tokenizer = Tokenizer::from_str(json);
        let symbols: Vec<Token> = tokenizer.into_iter().collect();
        let expected_symbols = vec![
            Token::ObjectOpen,
            Token::String("true".to_string()),
            Token::Colon,
            Token::True,
            Token::Comma,
            Token::String("false".to_string()),
            Token::Colon,
            Token::False,
            Token::Comma,
            Token::ObjectClose,
        ];

        assert_eq!(symbols, expected_symbols)
    }
}


#[cfg(test)]
mod parser {
    use std::collections::HashMap;

    use crate::{parse, JSON};

    fn assert_invalid(target: JSON, json: &'static str) {
        assert!(parse(json).is_err(), "Invalid JSON was parsed\n");
    }

    fn assert_parse(target: JSON, json: &'static str) {
        let result = parse(json).expect("Could not parse json\n");
        assert_eq!(target, result);
    }

    #[test]
    fn parses_null() {
        assert_parse(JSON::Null(), "null");
    }
    

    #[test]
    fn parses_int() {
        assert_parse(JSON::Number(100.), "100");
        assert_parse(JSON::Number(0.), "0");
    }

    #[test]
    fn parses_float() {
        assert_parse(JSON::Number(100.200), "100.2")
    }

    #[test]
    fn parses_string() {
        assert_parse(JSON::String("foobar".into()), "\"foobar\"")
    }

    #[test]
    fn parses_array() {
        let numbers = vec![1.,2.,3.,4.,5.].iter().map(|n| JSON::Number(*n)).collect();
        assert_parse(
            JSON::Array(numbers),
            "[1,2,3,4, 5]"
        )
    }

    #[test]
    fn parses_object() {
        let numbers = vec![1.,2.,3.,4.,5.].iter().map(|n| JSON::Number(*n)).collect();

        let mut object = HashMap::new();
        object.insert("foo".to_string(), JSON::Boolean(true));
        object.insert("bar".to_string(), JSON::Number(0.));
        object.insert("num".to_string(), JSON::Array(numbers));

        assert_parse(
            JSON::Object(object),
            r#"{
                    "foo": true,
                    "bar": 0,
                    "num": [1,2,3,4,5]
                }"#
        )
    }
}
