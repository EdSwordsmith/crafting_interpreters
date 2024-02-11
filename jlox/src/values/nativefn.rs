use std::fmt::Display;

use crate::{errors::RuntimeError, interpreter::Interpreter};

use super::{native_method, LoxCallable, LoxObj, LoxValue};

#[derive(Clone)]
pub struct NativeFn(
    pub usize,
    pub fn(&mut Interpreter, &[LoxObj]) -> Result<LoxObj, RuntimeError>,
    pub Option<LoxObj>,
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

    fn bind(&self, this: LoxObj) -> LoxObj {
        native_method(self.0, self.1, Some(this.clone()))
    }
}

impl LoxCallable for NativeFn {
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxObj]) -> Result<LoxObj, RuntimeError> {
        let mut args = args.to_vec();
        if let Some(this) = self.2.clone() {
            args.push(this);
        }
        self.1(interpreter, &args)
    }

    fn arity(&self) -> usize {
        self.0
    }
}
