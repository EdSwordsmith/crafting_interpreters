use std::{env, fs, println};

use errors::{Errors, ReportErrors};
use interpreter::Interpreter;
use rustyline::{DefaultEditor, Result as RLResult};

mod ast;
mod ast_printer;
mod errors;
mod interpreter;
mod parser;
mod scanner;
mod values;

fn run_file(path: &str) {
    let mut interpreter = Interpreter::new();
    let source = fs::read_to_string(path).unwrap();
    run(&mut interpreter, source).report_and_exit();
}

fn run_prompt() -> RLResult<()> {
    let mut interpreter = Interpreter::new();
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let line = rl.readline("> ")?;
        rl.add_history_entry(line.clone())?;
        run(&mut interpreter, line).report();
    }
}

fn run(interpreter: &mut Interpreter, source: String) -> Result<(), Errors> {
    let mut errors = Vec::new();
    let tokens = scanner::scan_tokens(source).unwrap(&mut errors);

    if !errors.is_empty() {
        return Err(Errors::Parsing(errors));
    }

    let statements = parser::parse(tokens)?;

    interpreter
        .interpret(&statements)
        .map_err(Errors::Runtime)?;

    Ok(())
}

fn main() -> RLResult<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt().unwrap_or(()),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: jlox [script]");
            std::process::exit(64);
        }
    };

    Ok(())
}
