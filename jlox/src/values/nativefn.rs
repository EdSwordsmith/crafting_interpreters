use std::fmt::Display;

use crate::{errors::RuntimeError, interpreter::Interpreter};

use super::{LoxCallable, LoxObj, LoxValue};

#[derive(Clone)]
pub struct NativeFn(
    pub usize,
    pub fn(&mut Interpreter, &[LoxObj]) -> Result<LoxObj, RuntimeError>,
);

impl Display for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}

impl LoxValue for NativeFn {
    fn callable(&self) -> Option<Box<dyn LoxCallable>> {
        Some(Box::new(self.clone()))
    }
}

impl LoxCallable for NativeFn {
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxObj]) -> Result<LoxObj, RuntimeError> {
        self.1(interpreter, args)
    }

    fn arity(&self) -> usize {
        self.0
    }
}
