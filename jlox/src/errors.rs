use std::fmt::Display;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub line: usize,
    pub message: String,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n[line {}]", self.message, self.line)
    }
}

pub enum Errors {
    Parsing(Vec<LoxError>),
    Runtime(RuntimeError),
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::Parsing(errors) => errors.iter().fold(Ok(()), |res, error| {
                res.and_then(|_| writeln!(f, "{error}"))
            }),
            Errors::Runtime(error) => writeln!(f, "{error}"),
        }
    }
}

pub fn error(line: usize, message: &str) -> LoxError {
    LoxError {
        line,
        location: String::new(),
        message: message.into(),
    }
}

pub fn error_with_location(line: usize, location: &impl AsRef<str>, message: &str) -> LoxError {
    LoxError {
        line,
        location: location.as_ref().into(),
        message: message.into(),
    }
}

pub trait ReportErrors {
    fn report(&self);
    fn report_and_exit(&self);
}

impl<T> ReportErrors for Result<T, Errors> {
    fn report(&self) {
        if let Err(errors) = self {
            eprint!("{}", errors);
        }
    }

    fn report_and_exit(&self) {
        if let Err(errors) = self {
            let code = match errors {
                Errors::Runtime(_) => 70,
                _ => 65,
            };

            eprint!("{}", errors);
            std::process::exit(code);
        }
    }
}
