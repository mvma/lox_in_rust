use core::f64;
use std::collections::HashMap;
use std::sync::OnceLock;

// A static, lazily-initialized, thread-safe HashMap that maps keyword strings to token types.
// `OnceLock` ensures that the HashMap is initialized only once, the first time it is accessed,
// making it ideal for a global, read-only reference. This allows efficient and safe access to
// a shared set of keywords used throughout the program.
static KEY_WORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

fn get_key_words() -> &'static HashMap<&'static str, TokenType> {
    KEY_WORDS.get_or_init(|| {
        let mut map = HashMap::new();

        map.insert("and", TokenType::And);
        map.insert("class", TokenType::Class);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("for", TokenType::For);
        map.insert("fun", TokenType::Fun);
        map.insert("if", TokenType::If);
        map.insert("nil", TokenType::Nil);
        map.insert("or", TokenType::Or);
        map.insert("print", TokenType::Print);
        map.insert("return", TokenType::Return);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("true", TokenType::True);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);

        map
    })
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

fn is_alpha(s: &str) -> bool {
    (s >= "a" && s <= "z") || (s >= "A" && s <= "Z") || s == "_"
}

fn is_digit(s: &str) -> bool {
    s >= "0" && s <= "9"
}

fn is_alpha_numeric(s: &str) -> bool {
    is_alpha(s) || is_digit(s)
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

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
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

        return &self.tokens;
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
                    // A comment goes until the end of the line.
                    while self.peek() != "\0" {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Nil);
                }
            }
            " " => return,
            "\r" | "\t" | "\n" => {
                self.line += 1;
                return;
            }
            "\"" => {
                self.string();
            }
            _ => {
                if is_digit(s) {
                    self.number();
                } else if is_alpha(s) {
                    self.identifier();
                } else {
                    panic!("Unrecognized token at line {}", self.line);
                }
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

        self.advance();

        let content: &str = &self.source[self.start + 1..self.current-1];

        self.add_token(TokenType::String, Literal::Text(content.into()));
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == "." && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let content: &str = &self.source[self.start..self.current];

        self.add_token(
            TokenType::Number,
            Literal::Number(content.parse::<f64>().unwrap()),
        );
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let content: &str = &self.source[self.start..self.current];

        // Attempt to retrieve the token type for the current identifier from the static keyword map.
        // If the identifier matches a keyword, get its corresponding TokenType. Otherwise, default
        // to TokenType::Identifier, treating it as a user-defined identifier. `cloned()` is used to
        // obtain an owned TokenType since `get` returns a reference.
        let key_words = get_key_words()
            .get(content)
            .cloned()
            .unwrap_or(TokenType::Identifier);

        self.add_token(key_words, Literal::Nil);
    }
}

#[derive(Clone, PartialEq)]
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

    pub fn lexeme(&self) -> &str {
        match self.token_type {
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Star => "*",
            TokenType::Slash => "/",
            TokenType::Bang => "!",
            TokenType::EqualEqual => "==",
            TokenType::BangEqual => "!=",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            _ => "",
        }
    }

    pub fn is_eof(&self) -> bool {
        self.token_type == TokenType::Eof
    }

    pub fn type_equals_to(&self, token_type: &TokenType) -> bool {
        &self.token_type == token_type
    }

    pub fn get_literal(&self) -> &Literal {
        &self.literal
    }

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }
}

#[derive(Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    Text(String),
    Nil,
    Boolean(bool),
}

impl Literal {
    pub fn to_custom_string(&self) -> String {
        match self {
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
            Literal::Number(n) => n.to_string(),
            Literal::Text(s) => s.clone(),
        }
    }
}

#[derive(Clone, PartialEq)]
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
