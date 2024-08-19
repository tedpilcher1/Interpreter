mod error_handling;
mod expressions;
mod parser;
mod printer;
mod scanner;
mod token_type;

use std::env;
use std::fs;
use std::process;

use token_type::Token;

fn run(source: &str) -> Result<(), String> {
    //  get tokens
    // just print for now

    return Err("Not implemented".to_string());
}

fn run_file(path: &str) -> Result<(), String> {
    // let bytes: Vec<u8> = fs::read(path).expect("Couldn't read file");
    // run(String::from_utf8(bytes).expect("Couldn't generate source string from bytes read"));

    match fs::read_to_string(path) {
        Ok(contents) => return run(&contents),
        Err(msg) => return Err(msg.to_string()),
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
