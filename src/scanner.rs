use std::{iter::{Iterator}};

pub struct Scanner{
    chars: Vec<char>,
    pointer: usize,
}

impl Scanner {
    pub fn from_str(value: &str) -> Self {
        Self {
            chars: value.chars().collect(),
            pointer: 0,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pointer)?;
        Some(ch.to_owned())
    }

    pub fn advance(&mut self){
        self.pointer += 1;
    }

    pub fn next(&mut self) -> Option<char> {
        let value = self.peek();
        self.advance();
        value
    }

}

impl Scanner {
    pub fn extract_token<T>(&mut self, expected_token: &str, value: T) -> Option<T> {
        self.extract_string(expected_token).map(|_| value)
    }

    pub fn extract_string(&mut self, expected_text: &str) -> Option<bool> {
        let mut text = String::new();
        for expected_ch in expected_text.chars() {
            let actual_ch = self.next()?;
            if actual_ch != expected_ch {
                return Some(false);
            }

            text.push(actual_ch);
        }

        Some(true)
    }

    pub fn extract_while_matching(&mut self, chars: &str) -> Option<String> {
        let mut text = String::new();
        
        loop {
            let next_ch = self.peek();
            match next_ch {
                None => { break },
                Some(ch) => {
                    if chars.chars().any(|x| x == ch) {
                        text.push(self.next()?);
                    } else {
                        break;
                    }
                }
            }
        }

        Some(text.to_string())
    }

    /// Returns "" if next char doesn't match
    pub fn extract_if_matching(&mut self, chars: &str) -> Option<String> {
        let next_ch = self.next()?;
        if chars.chars().any(|x| x == next_ch) {
            Some(next_ch.to_string())
        } else {
            Some("".to_string())
        }
    }
}