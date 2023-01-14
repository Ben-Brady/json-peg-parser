use crate::Error;
use crate::scanner::Scanner;
use std::fmt::Debug;
use std::iter::Iterator;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Colon,
    Comma,
    ObjectOpen,
    ObjectClose,
    ArrayOpen,
    ArrayClose,
    True,
    False,
    Null,
    String(String),
    Number(String),
}

pub struct Tokenizer {
    scanner: Scanner,
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().ok()
    }
}

impl Tokenizer {
    pub fn new(value: &str) -> Tokenizer {
        Tokenizer {
            scanner: Scanner::new(value)
        }
    }
    
    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.scanner.possible_many_characters(" \r\n\t")?;
        self.scanner.pop_buffer();
        let next_ch = self.scanner.peek().ok_or(Error::UnexpectedEnd)?;

        let token = match next_ch {
            ',' => self.next_symbol(",", Token::Comma),
            ':' => self.next_symbol(":", Token::Colon),
            '{' => self.next_symbol("{", Token::ObjectOpen),
            '}' => self.next_symbol("}", Token::ObjectClose),
            '[' => self.next_symbol("[", Token::ArrayOpen),
            ']' => self.next_symbol("]", Token::ArrayClose),
            't' => self.next_symbol("true", Token::True),
            'f' => self.next_symbol("false", Token::False),
            'n' => self.next_symbol("null", Token::Null),
            '"' => self.next_string(),
            '-'|'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => self.next_number(),
            _ => { Err(Error::InvalidSyntax)},
        }?;
        Ok(token)
    }
}

impl Tokenizer {
    pub fn next_symbol<T>(&mut self, expected_token: &str, value: T) -> Result<T, Error> {
        self.scanner.take_string(expected_token)?;
        Ok(value)
    }

    fn next_string(&mut self) -> Result<Token, Error> {
        self.scanner.possible_single_characters("\"")?;
        loop {
            let ch = self.scanner.expect_single_any()?;

            if ch == '"' {
                break;
            } else if ch == '\\' {
                let next_ch = self.scanner.peek().ok_or(Error::InvalidSyntax)?;
                match next_ch {
                    '"'|'\\'|'/'|'n'|'r'|'t'|'b'|'f'|'u' => {
                        self.scanner.expect_single_any()?;
                    },
                    _ => Err(Error::InvalidSyntax)?,
                }
            }
        }

        let text = self.scanner.pop_buffer();
        Ok(Token::String(text))
    }

    fn next_number(&mut self) -> Result<Token, Error> {
        self.scanner.possible_single_characters("-")?;
        match self.scanner.expect_single_any()? {
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                self.scanner.possible_many_characters("1234567890")?;
                if self.scanner.peek() == Some('.') {
                    self.next_number_take_decimal()?;
                }
            },
            '0' => {
                match self.scanner.peek() {
                    None => { },
                    Some(ch) => {
                        match ch {
                            'e'|'E' => {
                                self.scanner.expect_single_characters("eE")?;
                                self.scanner.expect_single_characters("-+")?;
                                self.scanner.expect_many_characters("1234567890")?;
                            },
                            '.' => { self.next_number_take_decimal()?; },
                            _ => {}
                        }
                    }
                }
            },
            _ => { return Err(Error::InvalidSyntax)}
        };

        let number = self.scanner.pop_buffer();
        Ok(Token::Number(number))
    }

    fn next_number_take_decimal(&mut self) ->Result<(), Error> {
        self.scanner.expect_single_any()?; // Take the .
        self.scanner.expect_many_characters("1234567890")?;
        Ok(())
    }
}
