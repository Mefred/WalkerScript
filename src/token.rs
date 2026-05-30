use crate::token::TokenType::{
    CloseBrace, CloseParen, Comma, Dot, Equal, Minus, OpenBrace, OpenParen, Plus, Semicolon, Star,
};

pub enum TokenType {
    // Keywords
    Let,
    Fn,
    If,
    Else,
    For,
    While,

    // Literals
    Identifier,
    String,
    Number,

    // Character(s)
    Plus,
    Minus,
    Mult,
    Power,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Dot,
    Semicolon,
    Slash,
    Star,

    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    EOF,
}

enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line: line,
        }
    }
}

pub struct Scanner {
    src: Vec<char>,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            src: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.src.len() as i32;
    }

    pub fn scan_tokens(&mut self) -> Vec<TokenType> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            None,
            self.line as usize,
        ));
        vec![]
    }

    fn advance(&mut self) -> char {
        let temp = self.src[self.current as usize];
        self.current += 1;
        return temp;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self.src[self.start as usize..self.current as usize]
            .iter()
            .collect();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line as usize))
    }

    fn check(&mut self, expacted: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.src[self.current as usize] != expacted {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(OpenParen, None),
            ')' => self.add_token(CloseParen, None),
            '{' => self.add_token(OpenBrace, None),
            '}' => self.add_token(CloseBrace, None),
            ',' => self.add_token(Comma, None),
            '.' => self.add_token(Dot, None),
            '+' => self.add_token(Plus, None),
            '-' => self.add_token(Minus, None),
            ';' => self.add_token(Semicolon, None),
            '*' => self.add_token(Star, None),

            '=' => self.add_token(Equal, None),

            _ => panic!("invalid token"),
        }
    }
}
