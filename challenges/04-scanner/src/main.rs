use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

use crate::scanner::Scanner;

mod scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox { had_error: false }
    }

    fn run_file(&mut self, path: &str) {
        let source = fs::read_to_string(path).unwrap();
        self.run(source);
        if self.had_error {
            std::process::exit(65);
        }
    }

    fn run_prompt(&mut self) {
        loop {
            print!("> ");
            stdout().flush().unwrap();

            let mut line = String::new();
            if stdin().read_line(&mut line).unwrap() <= 1 {
                break;
            }

            self.run(line);
            self.had_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(self, source);
        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();

    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]),
        _ => {
            println!("Usage: jlox [script]");
            std::process::exit(64);
        }
    }
}
