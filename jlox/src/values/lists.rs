use std::fmt::Display;

use crate::{errors::RuntimeError, interpreter::Interpreter, scanner::Token};

use super::{native_fn, nil, number, LoxObj, LoxProperty, LoxValue};

pub struct LoxList(pub Vec<LoxObj>);

impl Display for LoxList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .0
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{items}]")
    }
}

impl LoxValue for LoxList {
    fn is_truthy(&self) -> bool {
        !self.0.is_empty()
    }

    fn get_property(&self, name: &Token) -> LoxProperty {
        match name.lexeme.as_str() {
            "push" => LoxProperty::Method(native_fn(1, push)),
            "pop" => LoxProperty::Method(native_fn(0, pop)),
            "len" => LoxProperty::Field(number(self.0.len() as f64)),
            _ => LoxProperty::Undef,
        }
    }

    fn push(&mut self, item: LoxObj) {
        self.0.push(item.clone())
    }

    fn pop(&mut self) -> Option<LoxObj> {
        self.0.pop()
    }
}

fn push(_interpreter: &mut Interpreter, args: &[LoxObj]) -> Result<LoxObj, RuntimeError> {
    let list = args[1].clone();
    list.0.borrow_mut().push(args[0].clone());
    Ok(nil())
}

fn pop(_interpreter: &mut Interpreter, args: &[LoxObj]) -> Result<LoxObj, RuntimeError> {
    let list = args[0].clone();
    list.0.clone().borrow_mut().pop().ok_or(RuntimeError {
        line: 0,
        message: "Cannot pop from empty list".into(),
    })
}
