use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two characters
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier(String),
    String(String),
    Number(f64),

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub struct Token {
    pub t: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(t: TokenType, lexeme: String, line: usize) -> Token {
        Token { t, lexeme, line }
    }
}

impl<'a> fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.t, self.lexeme)
    }
}
