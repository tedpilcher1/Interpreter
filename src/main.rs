mod error_handling;
mod expressions;
mod parser;
mod printer;
mod scanner;
mod token_type;

use std::env;
use std::fs;
use std::process;

use parser::Parser;
use scanner::Scanner;
use token_type::Token;

fn run(source: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(source.to_string());
    scanner.scan_tokens();

    let tokens = scanner.tokens;
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    println!("{}", expression.to_string());

    return Ok(());
}

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Ok(contents) => return run(&contents),
        Err(msg) => Err(msg.to_string()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => process::exit(0),
            Err(msg) => {
                println!("Error: {}", msg);
                process::exit(1);
            }
        }
    }
}
