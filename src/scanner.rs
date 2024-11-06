use core::f64;
use std::{fmt, str};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            source: s,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Literal::Nil,
            self.line,
        ));

        return vec![];
    }

    fn scan_token(&mut self) {
        let s = self.advance();

        match s {
            "(" => self.add_token(TokenType::LeftParen, Literal::Nil),
            ")" => self.add_token(TokenType::RightParen, Literal::Nil),
            "{" => self.add_token(TokenType::LeftBrace, Literal::Nil),
            "}" => self.add_token(TokenType::RightBrace, Literal::Nil),
            "," => self.add_token(TokenType::Comma, Literal::Nil),
            ";" => self.add_token(TokenType::Semicolon, Literal::Nil),
            "." => self.add_token(TokenType::Dot, Literal::Nil),
            "-" => self.add_token(TokenType::Minus, Literal::Nil),
            "+" => self.add_token(TokenType::Plus, Literal::Nil),
            "*" => self.add_token(TokenType::Star, Literal::Nil),
            "!" => {
                let token_type = if self.match_next("=") {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(token_type, Literal::Nil)
            }
            "=" => {
                let token_type = if self.match_next("=") {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token_type, Literal::Nil)
            }
            "<" => {
                let token_type = if self.match_next("=") {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                self.add_token(token_type, Literal::Nil)
            }
            ">" => {
                let token_type = if self.match_next("=") {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(token_type, Literal::Nil)
            }
            "/" => {
                if self.match_next("/") {
                    // A comment goes until the end of the line
                    while self.peek() != "\0" {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Nil);
                }
            }
            " " | "\r" | "\t" | "\n" => {
                self.line += 1;
                return;
            }
            "\"" => {
                self.string();
            }
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                self.number();
            }
            _ => {
                panic!("Unrecognized token at line {}", self.line);
            }
        };
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> &str {
        self.current += 1;
        return &self.source[self.current - 1..self.current];
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let lexeme = &self.source[self.start..self.current];

        let token = Token::new(token_type, lexeme.to_string(), literal, self.line);

        println!("{}", token);

        self.tokens.push(token);
    }

    fn match_next(&mut self, exp: &str) -> bool {
        if self.is_at_end() {
            return false;
        }

        let next = &self.source[self.current..self.current + 1];

        if next != exp {
            return false;
        }

        self.current += 1;

        return true;
    }

    fn peek(&self) -> &str {
        if self.is_at_end() {
            return "\0";
        }
        &self.source[self.current..self.current + 1]
    }

    fn peek_next(&self) -> &str {
        if self.current + 1 >= self.source.len() {
            return "\0";
        }
        &self.source[self.current + 1..self.current + 2]
    }

    fn is_digit(&self, s: &str) -> bool {
        s >= "0" && s <= "9"
    }

    fn string(&mut self) {
        while self.peek() != "\"" && !self.is_at_end() {
            if self.peek() == "\n" {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string at {}", self.line);
        }

        // Move to the closing ""
        self.advance();

        let content: &str = &self.source[self.start + 1..self.current];

        self.add_token(TokenType::String, Literal::Text(content.into()));
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == "." && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let content: &str = &self.source[self.start..self.current];

        self.add_token(
            TokenType::Number,
            Literal::Number(content.parse::<f64>().unwrap()),
        );
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: u32) -> Self {
        Self {
            token_type: (token_type),
            lexeme: (lexeme),
            literal: (literal),
            line: (line),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    Text(String),
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub enum TokenType {
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

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

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
