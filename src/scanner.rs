use crate::Error;
use std::{iter::{Iterator}};

pub struct Scanner {
    chars: Vec<char>,
    pointer: usize,
    buffer: String,
}

impl Scanner {
    pub fn new(value: &str) -> Self {
        Self {
            chars: value.chars().collect(),
            pointer: 0,
            buffer: String::new()
        }
    }
    
    pub fn peek(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pointer)?;
        Some(ch.to_owned())
    }
    
    pub fn pop_buffer(&mut self) -> String {
        let token = self.buffer.clone();
        self.buffer.clear();
        token
    }

    pub fn expect_single_any(&mut self) -> Result<char, Error> {
        match self.peek() {
            None => Err(Error::UnexpectedEnd),
            Some(ch) => {
                self.advance(1);
                self.buffer.push(ch);
                Ok(ch)
            }
        }
    }
}


impl Scanner {
    fn advance(&mut self, offset: usize) {
        self.pointer += offset;
    }

    
    pub fn possible_single(&mut self, func: Box<dyn Fn(char) -> bool + Send + 'static>) -> Result<(), Error> {
        match self.peek() {
            None => Err(Error::UnexpectedEnd),
            Some(ch) => {
                if func(ch) {
                    self.expect_single_any()?;
                }
                Ok(())
            },
        }
    }
    
    pub fn expect_single(&mut self, func: Box<dyn Fn(char) -> bool + Send + 'static>) -> Result<(), Error> {
        match self.peek() {
            None => Err(Error::UnexpectedEnd),
            Some(ch) => {
                if func(ch) {
                    self.expect_single_any()?;
                    Ok(())
                 } else {
                    Err(Error::InvalidSyntax)
                 }
            },
        }
    }
    
    pub fn possible_many(&mut self, func: Box<dyn Fn(char) -> bool + Send + 'static>) -> Result<(), Error> {
        loop {
            match self.peek() {
                None => { break; },
                Some(ch) => {
                    if func(ch) {
                        self.expect_single_any()?;
                    } else {
                        break;
                    }
                },
            }
        };

        Ok(())
    }

    pub fn possible_single_characters(&mut self, chars: &'static str) -> Result<(), Error>{
        let func = |ch| chars.chars().any(|x| x == ch);
        self.possible_single(Box::new(func))?;
        Ok(())
    }

    pub fn possible_many_characters(&mut self, chars: &'static str) -> Result<(), Error>{
        let func = |ch| chars.chars().any(|x| x == ch);
        self.possible_many(Box::new(func))?;
        Ok(())
    }

    pub fn expect_single_characters(&mut self, chars: &'static str) -> Result<(), Error> {
        let char = self.expect_single_any()?;
        if chars.chars().any(|x| x == char) {
            Ok(())
        } else {
            Err(Error::InvalidSyntax)
        }
    }
    
    pub fn expect_many_characters(&mut self, chars: &'static str) -> Result<(), Error> {
        self.expect_single_characters(chars)?;
        self.possible_many_characters(chars)?;
        Ok(())
    }
    
    pub fn take_string(&mut self, expected_text: &str) -> Result<(), Error> {
        let start = self.pointer;
        let end = self.pointer + expected_text.len();
        let slice: String = self.chars[start..end].iter().collect();
        
        if slice == expected_text {
            self.advance(expected_text.len());
            Ok(())
        } else {
            Err(Error::InvalidSyntax)
        }
    }
}