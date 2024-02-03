use crate::{errors::RuntimeError, interpreter::Interpreter};

use super::{LoxObj, LoxValue};

pub trait LoxCallable: LoxValue {
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxObj]) -> Result<LoxObj, RuntimeError>;

    fn arity(&self) -> usize;
}
