pub mod types;

use types::Token;

pub struct Scanner<'a> {
    start: usize,
    current: usize,
    src: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            start: 0,
            current: 0,
            src: source,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut _tokens: Vec<Token> = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token() {
                _tokens.push(token);
            }
        }
        _tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c: u8 = self.eat();

        if c.is_ascii_digit() {
            return Some(self.number());
        }

        match c {
            b'+' => Some(Token::PLUS),
            b'-' => Some(Token::MINUS),
            b'*' => Some(Token::MULTIPLY),
            b'/' => Some(Token::DIVIDE),
            b'^' => Some(Token::EXPONENT),
            x => {
                if x.is_ascii_whitespace() {
                    None
                } else {
                    panic!("Unexpected token '{}'", char::from(c))
                }
            }
        }
    }

    fn number(&mut self) -> Token {
        let mut is_float = false;
        while self.peek(0).is_ascii_digit() {
            self.eat();
        }
        if self.peek(0) == b'.' {
            is_float = true;
            self.eat();

            while self.peek(0).is_ascii_digit() {
                self.eat();
            }
        }

        if !is_float {
            Token::INTEGER {
                val: self.src[self.start..self.current].parse::<i64>().unwrap(),
            }
        } else {
            Token::FLOAT {
                val: self.src[self.start..self.current].parse::<f64>().unwrap(),
            }
        }
    }

    fn eat(&mut self) -> u8 {
        self.current += 1;
        self.src.as_bytes()[self.current - 1]
    }

    fn peek(&self, offset: usize) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.src.as_bytes()[self.current + offset]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }
}
