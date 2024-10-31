use std::fmt;

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
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Literal::Nil,
            self.line,
        ));

        return vec![];
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(&self) {}

    fn advance(&mut self) -> &str {
        self.current += 1;
        return &self.source[self.current - 1..self.current];
    }

    fn add_token(&self, token_type: TokenType, literal: Literal) {}
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

#[derive(Debug)]
enum Literal {
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
