mod scanner;
use crate::scanner::*;

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

    for token in tokens {
        println!("{:?}", token);
    }
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