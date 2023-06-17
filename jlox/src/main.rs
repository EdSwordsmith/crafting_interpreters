use std::{env, fs, println};

use errors::{Errors, ReportErrors};
use rustyline::{DefaultEditor, Result as RLResult};

use crate::{ast::ExprVisitor, ast_printer::AstPrinter};

mod ast;
mod ast_printer;
mod errors;
mod parser;
mod scanner;

fn run_file(path: &str) {
    let source = fs::read_to_string(path).unwrap();
    run(source).report_and_exit(65);
}

fn run_prompt() -> RLResult<()> {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let line = rl.readline("> ")?;
        rl.add_history_entry(line.clone())?;
        run(line).report();
    }
}

fn run(source: String) -> Result<(), Errors> {
    let mut errors = Vec::new();
    let tokens = scanner::scan_tokens(source).unwrap(&mut errors);

    if !errors.is_empty() {
        return Err(Errors(errors));
    }

    let expr = parser::parse(tokens)?;
    println!("{:?}", AstPrinter.visit(&expr));

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
