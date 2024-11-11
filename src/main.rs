mod scanner;
use crate::scanner::*;
mod parser;
use crate::parser::*;

use std::env;
use std::fs;
use std::io;
use std::process;

fn main() {
    const TOTAL_ARGS_EXPECTED: usize = 2;

    let args: Vec<String> = env::args().collect();

    if args.len() > TOTAL_ARGS_EXPECTED {
        println!("Usage rlox [script]!");
        process::exit(64);
    } else if args.len() == TOTAL_ARGS_EXPECTED {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(s: &str) {
    let file_content = fs::read_to_string(&s).expect("Could not read the file");

    run(&file_content);
}

fn run(s: &str) {
    let mut scanner = Scanner::new(s);

    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.to_vec());
    parser.expression();
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

        run(&line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        use crate::scanner::{Literal, Token, TokenType};

        let tokens = vec![
            Token::new(TokenType::Number, "2".to_string(), Literal::Number(2.0), 1),
            Token::new(TokenType::Plus, "+".to_string(), Literal::Nil, 1),
            Token::new(TokenType::Number, "2".to_string(), Literal::Number(2.0), 1),
            Token::new(TokenType::Eof, "".to_string(), Literal::Nil, 1), // Important: Include EOF
        ];

        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert_eq!(expression.to_string(), "(+ 2 2)");
    }
}
