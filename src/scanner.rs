use crate::token::{Token, TokenType};
use anyhow::Result;

pub struct Scanner {
    start: usize,
    current: usize,
    line: usize,
    source: Vec<u8>,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // at the start of next lexeme
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));
        self.tokens.as_ref()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => eprintln!("Unexpected character"), //TODO return custom error type
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current] as char;
        self.current += 1;
        c
    }

    fn add_token(&mut self, t: TokenType) {
        let lexeme = std::str::from_utf8(&self.source[self.start..self.current])
            .expect("Invalid utf-8 sequence");
        self.tokens
            .push(Token::new(t, lexeme.to_string(), self.line));
    }
}
