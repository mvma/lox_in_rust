mod scanner;
use crate::scanner::*;
mod parser;
use crate::parser::*;
mod interpreter;
use crate::interpreter::*;
mod environment;
use crate::environment::*;

use std::env;
use std::fs;
use std::io;
use std::process;

fn main() {
    const TOTAL_ARGS_EXPECTED: usize = 2;

    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&TOTAL_ARGS_EXPECTED) {
        std::cmp::Ordering::Greater => {
            println!("Usage: rlox [script]!");
            process::exit(64);
        }
        std::cmp::Ordering::Equal => {
            run_file(&args[1]);
        }
        std::cmp::Ordering::Less => {
            run_prompt();
        }
    }
}

fn run_file(s: &str) {
    let file_content = fs::read_to_string(s).expect("Could not read the file");

    run(&file_content);
}

fn run(s: &str) {
    let mut scanner = Scanner::new(s);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.to_vec());
    let statements = parser.parse();

    let mut interpreter = Interpreter::new(Environment::new());
    interpreter.interpret(statements);
}

fn run_prompt() {
    loop {
        println!("> ");

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Could not read the line");

        let line = line.trim();

        if line.is_empty() {
            break;
        }

        run(line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_add() {
        use crate::scanner::{Literal, Token, TokenType};

        let tokens = vec![
            Token::new(TokenType::Number, "2".to_string(), Literal::Number(2.0), 1),
            Token::new(TokenType::Plus, "+".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "2".to_string(), Literal::Number(2.0), 1),
            Token::new(TokenType::Eof, "".to_string(), Literal::Nil, 1),
        ];

        let mut parser = Parser::new(tokens);
        let expression = parser.parse_expression();

        assert_eq!(expression.to_custom_string(), "(+ 2 2)");
    }

    #[test]
    fn it_parses_with_precedence() {
        use crate::scanner::{Literal, Token, TokenType};

        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), Literal::Number(1.0), 1),
            Token::new(TokenType::Plus, "+".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "2".to_string(), Literal::Number(2.0), 1),
            Token::new(TokenType::Star, "*".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "3".to_string(), Literal::Number(3.0), 1),
            Token::new(TokenType::Eof, "".to_string(), Literal::Nil, 1),
        ];

        let mut parser = Parser::new(tokens);
        let expression = parser.parse_expression();

        assert_eq!(expression.to_custom_string(), "(+ 1 (* 2 3))");
    }

    #[test]
    fn it_parses_with_grouping() {
        use crate::scanner::{Literal, Token, TokenType};

        let tokens = vec![
            Token::new(TokenType::LeftParen, "(".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "1".to_string(), Literal::Number(1.0), 1),
            Token::new(TokenType::Plus, "+".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "2".to_string(), Literal::Number(2.0), 1),
            Token::new(TokenType::RightParen, ")".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Star, "*".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "3".to_string(), Literal::Number(3.0), 1),
            Token::new(TokenType::Eof, "".to_string(), Literal::Nil, 1),
        ];

        let mut parser = Parser::new(tokens);
        let expression = parser.parse_expression();

        assert_eq!(expression.to_custom_string(), "(* (group (+ 1 2)) 3)");
    }

    #[test]
    #[should_panic(expected = "Expect ')' after expression.")]
    fn it_should_panic_missing_paren() {
        use crate::scanner::{Literal, Token, TokenType};

        let tokens = vec![
            Token::new(TokenType::LeftParen, "(".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "1".to_string(), Literal::Number(1.0), 1),
            Token::new(TokenType::Plus, "+".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "2".to_string(), Literal::Number(2.0), 1),
            Token::new(TokenType::Star, "*".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "3".to_string(), Literal::Number(3.0), 1),
            Token::new(TokenType::Eof, "".to_string(), Literal::Nil, 1),
        ];

        let mut parser = Parser::new(tokens);
        parser.parse_expression();
    }

    #[test]
    #[should_panic]
    fn it_should_panic_accentuation() {
        let mut scanner = Scanner::new("print \"é\"");
        scanner.scan_tokens();
    }

    #[test]
    fn it_should_tokenize() {
        let mut scanner = Scanner::new("print \"e\"");
        let tokens = scanner.scan_tokens().to_vec();

        assert_eq!(tokens[0].type_equals_to(&TokenType::Print), true);
        assert_eq!(tokens[1].type_equals_to(&TokenType::String), true);
    }

    #[test]
    fn it_computes() {
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), Literal::Number(1.0), 1),
            Token::new(TokenType::Plus, "".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "1".to_string(), Literal::Number(1.0), 1),
            Token::new(TokenType::Eof, "".to_string(), Literal::Nil, 1),
        ];

        let mut parser = Parser::new(tokens);
        let mut environment = Environment::new();

        let literal = parser.parse_expression().evaluate(&mut environment);

        assert_eq!(literal.to_string(), "2");
    }

    #[test]
    fn it_manages_environments() {
        let mut current_environment = Environment::new();
        current_environment.define("i", Literal::Number(1.0));
        current_environment.define("j", Literal::Number(2.0));

        let mut inner = Environment::new_with_enclosing(Some(Box::from(current_environment)));
        inner.define("i", Literal::Number(2.0));

        let mut inner_most = Environment::new_with_enclosing(Some(Box::from(inner)));
        inner_most.define("k", Literal::Text("John".to_string()));

        let variable_i = inner_most.get("i");
        let variable_j = inner_most.get("j");
        let variable_k = inner_most.get("k");

        assert_eq!(variable_i.unwrap(), &Literal::Number(2.0));
        assert_eq!(variable_j.unwrap(), &Literal::Number(2.0));
        assert_eq!(variable_k.unwrap(), &Literal::Text("John".to_string()));
    }
}
