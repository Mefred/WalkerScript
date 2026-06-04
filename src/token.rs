use crate::token::TokenType::{
    And, CloseBrace, CloseParen, Comma, Dot, Else, Equal, EqualEqual, False, Fn, For, Greater,
    GreaterEqual, Identifier, If, Less, LessEqual, Let, Minus, Not, NotEqual, Number, OpenBrace,
    OpenParen, Or, Plus, Print, Return, Semicolon, Slash, Star, StringLit, True, While,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum TokenType {
    // Keywords
    Let,
    Fn,
    If,
    Else,
    For,
    While,
    And,
    False,
    Print,
    Return,
    True,
    Or,

    // Literals
    Identifier,
    StringLit,
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

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Token {
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

#[derive(Debug, Clone)]
pub struct Scanner {
    src: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            src: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                (String::from("and"), And),
                (String::from("else"), Else),
                (String::from("false"), False),
                (String::from("for"), For),
                (String::from("fn"), Fn),
                (String::from("if"), If),
                (String::from("or"), Or),
                (String::from("print"), Print),
                (String::from("return"), Return),
                (String::from("true"), True),
                (String::from("let"), Let),
                (String::from("while"), While),
            ]),
        }
    }

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.src.len();
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        return self.tokens.clone();
    }

    fn advance(&mut self) -> char {
        let temp = self.src[self.current];
        self.current += 1;
        return temp;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self.src[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    fn check(&mut self, expacted: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.src[self.current] != expacted {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.src[self.current];
    }

    fn string_fn(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            panic!("unterminated string")
        }

        self.advance();

        let value: String = self.src[self.start + 1..self.current - 1].iter().collect();
        self.add_token(StringLit, Some(Literal::String(value)));
    }

    fn isDigit(&mut self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.src.len() {
            return '\0';
        }
        return self.src[self.current + 1];
    }

    fn is_alpha(&mut self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alpha_numeric(&mut self, c: char) -> bool {
        return self.is_alpha(c) || self.isDigit(c);
    }

    fn number(&mut self) {
        let mut temp = self.peek();
        while self.isDigit(temp) {
            self.advance();
            temp = self.peek();
        }
        temp = self.peek_next();
        if self.peek() == '.' && self.isDigit(temp) {
            self.advance();
            while self.isDigit(temp) {
                self.advance();
                temp = self.peek();
            }
        }

        let number_str: String = self.src[self.start..self.current].iter().collect();

        self.add_token(Number, Some(Literal::Number(number_str.parse().unwrap())));
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

            '!' => {
                let token_type = if self.check('=') { NotEqual } else { Not };

                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.check('=') { EqualEqual } else { Equal };

                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.check('=') { LessEqual } else { Less };

                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.check('=') {
                    GreaterEqual
                } else {
                    Greater
                };

                self.add_token(token_type, None);
            }

            '/' => {
                if self.check('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash, None);
                }
            }

            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string_fn(),

            _ => {
                if self.isDigit(c) {
                    self.number()
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    panic!("invalid token {}", c)
                }
            }
        }
    }

    fn identifier(&mut self) {
        let mut temp = self.peek();
        while self.is_alpha_numeric(temp) {
            self.advance();
            temp = self.peek();
        }
        let text: String = self.src[self.start..self.current].iter().collect();
        if self.keywords.get(&text).is_none() {
            self.add_token(Identifier, None);
            return;
        }
        let type_identifier: TokenType = self.keywords.get(&text).unwrap().clone();
        self.add_token(type_identifier, None);
    }
}
