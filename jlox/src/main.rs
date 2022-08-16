use std::{
    env, fs,
    io::{stdin, stdout, BufRead, Write},
};

use errors::{Errors, ReportErrors};
use crate::scanner::{Token, TokenType};

mod ast;
mod ast_printer;
mod errors;
mod scanner;

fn run_file(path: &str) {
    let source = fs::read_to_string(path).unwrap();
    run(source).report_and_exit(65);
}

fn run_prompt() {
    print!("> ");
    stdout().flush().unwrap();

    for line in stdin().lock().lines() {
        let line = line.unwrap();
        run(line).report();

        print!("> ");
        stdout().flush().unwrap();
    }
}

fn run(source: String) -> Result<(), Errors> {
    let mut errors = Vec::new();
    let tokens = scanner::scan_tokens(source).unwrap(&mut errors);

    println!("{:?}", tokens);
    if !errors.is_empty() {
        return Err(Errors(errors));
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: jlox [script]");
            std::process::exit(64);
        }
    }
}
