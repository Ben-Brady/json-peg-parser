use crate::scanner::{Scanner};
use std::fmt::Debug;
use std::str::Chars;
use std::iter::{Iterator, Peekable};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    ObjectOpen,
    Colon,
    ObjectClose,
    
    ArrayOpen,
    ArrayClose,

    Comma,

    False,
    True,
    Null,
    String(String),
    Number(String),
}

// type PeekableIterator = Peekable<Box<Chars <'static>>>;

pub struct Tokenizer {
    scanner: Scanner,
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl Tokenizer {
    pub fn from_str(value: &str) -> Tokenizer {
        Tokenizer {
            scanner: Scanner::from_str(value)
        }
    }
    
    pub fn next_token(&mut self) -> Option<Token> {
        self.scanner.extract_while_matching(" \r\n\t");
        let next_ch = self.scanner.peek()?;
        let token = match next_ch {
            ',' => self.scanner.extract_token(",", Token::Comma),
            ':' => self.scanner.extract_token(":", Token::Colon),
            '{' => self.scanner.extract_token("{", Token::ObjectOpen),
            '}' => self.scanner.extract_token("}", Token::ObjectClose),
            '[' => self.scanner.extract_token("[", Token::ArrayOpen),
            ']' => self.scanner.extract_token("]", Token::ArrayClose),
            't' => self.scanner.extract_token("true", Token::True),
            'f' => self.scanner.extract_token("false", Token::False),
            'n' => self.scanner.extract_token("null", Token::Null),
            '"' => self.next_string(),
            '-'|'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => self.next_number(),
            _ => None,
        };
        println!("{:?}", token);
        token
    }

    fn next_string(&mut self) -> Option<Token> {
        let mut string = String::new();
        self.scanner.extract_string("\"")?;
        
        loop {
            let ch = self.scanner.next()?;

            if ch == '"' {
                break;
            }
            
            if ch != '\\' {
                string.push(ch);
                continue;
            }

            string.push('\\');
            let next_ch = self.scanner.peek()?;
            match next_ch {
                ch@('"'|'\\'|'/'|'n'|'r'|'t'|'b'|'f'|'u') => {
                    self.scanner.advance();
                    string.push(ch);
                },
                _ => {
                    return None;
                },
            }
        }
        
        Some(Token::String(string))
    }
    

    fn next_number(&mut self) -> Option<Token> {
        let mut text = String::new();
        if self.scanner.peek()? == '-' {
            self.scanner.advance();
            text.push('-');
        }
        

        let next_ch = self.scanner.next()?;
        text.push(next_ch);
        match next_ch {
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                let numbers = self.scanner.extract_while_matching("1234567890")?;
                text.push_str(numbers.as_str());
                if self.scanner.peek() == Some('.') {
                    text.push(self.scanner.next()?);
                    let numbers = self.scanner.extract_while_matching("1234567890")?;
                    text.push_str(numbers.as_str());
                }
                Some(Token::Number(text))
            },
            '0' => {
                match self.scanner.peek() {
                    None => Some(Token::Number(text)),
                    Some(ch) => match ch {
                        'e'|'E' => {
                            text.push(self.scanner.next()?);
                            let plus_minus = self.scanner.extract_if_matching("-+")?;
                            text.push_str(plus_minus.as_str());
                            text.push_str(self.scanner.extract_while_matching("1234567890")?.as_str());
                            Some(Token::Number(text))
                        },
                        '.' => {
                            text.push(self.scanner.next()?);
                            text.push_str(self.scanner.extract_while_matching("1234567890")?.as_str());
                            Some(Token::Number(text))
                        },
                        _ => Some(Token::Number(text))
                    }
                }
            },
            _ => None
        }
    }
}
