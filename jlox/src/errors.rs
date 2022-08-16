use std::fmt::Display;

pub struct LoxError {
    line: usize,
    location: String,
    message: String,
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line {}] Error{}: {}",
            self.line, self.location, self.message
        )
    }
}

pub struct Errors(pub Vec<LoxError>);

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |res, error| {
            res.and_then(|_| writeln!(f, "{}", error))
        })
    }
}

pub fn error(line: usize, message: &str) -> LoxError {
    LoxError {
        line,
        location: String::new(),
        message: message.into(),
    }
}

pub trait ReportErrors {
    fn report(&self);
    fn report_and_exit(&self, code: i32);
}

impl<T> ReportErrors for Result<T, Errors> {
    fn report(&self) {
        if let Err(errors) = self {
            eprint!("{}", errors);
        }
    }

    fn report_and_exit(&self, code: i32) {
        if let Err(errors) = self {
            eprint!("{}", errors);
            std::process::exit(code);
        }
    }
}
