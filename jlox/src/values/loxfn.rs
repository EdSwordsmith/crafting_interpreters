use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{ast::Stmt, interpreter::Environment};

use super::{lox_fn, native_fn, nil, LoxCallable, LoxObj, LoxValue};

#[derive(Clone)]
pub struct LoxFn(pub Box<Stmt>, pub Rc<RefCell<Environment>>, pub bool);

impl Display for LoxFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Stmt::Function { name, .. } = *self.0.clone() {
            write!(f, "<fn {}>", name.lexeme)
        } else {
            unreachable!()
        }
    }
}

impl LoxValue for LoxFn {
    fn callable(&self) -> Option<Box<dyn LoxCallable>> {
        Some(Box::new(self.clone()))
    }

    fn bind(&self, this: LoxObj, inner: LoxObj) -> LoxObj {
        let closure = Environment::with_enclosing(self.1.clone());
        closure.borrow_mut().define("this".into(), this);
        closure.borrow_mut().define("inner".into(), inner);
        lox_fn(self.0.clone(), closure, self.2)
    }
}

impl LoxCallable for LoxFn {
    fn call(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        args: &[LoxObj],
    ) -> Result<LoxObj, crate::errors::RuntimeError> {
        if let Stmt::Function { params, body, .. } = *self.0.clone() {
            let environment = Environment::with_enclosing(self.1.clone());
            for (param, value) in params.iter().zip(args.iter()) {
                environment
                    .borrow_mut()
                    .define(param.lexeme.clone(), value.clone());
            }

            let res = interpreter.execute_block(&body, environment)?;

            if self.2 {
                self.1.borrow().get_at(0, "this".into())
            } else {
                Ok(res.unwrap_or(nil()))
            }
        } else {
            unreachable!()
        }
    }

    fn arity(&self) -> usize {
        if let Stmt::Function { params, .. } = *self.0.clone() {
            params.len()
        } else {
            unreachable!()
        }
    }
}

pub trait Methods {
    fn bind(&self, this: LoxObj) -> LoxObj;
}

impl Methods for Vec<LoxObj> {
    fn bind(&self, this: LoxObj) -> LoxObj {
        let mut last = None;
        for method in self.iter().rev() {
            let inner = last.unwrap_or(native_fn(0, |_, _| Ok(nil())));
            last = Some(method.bind(this.clone(), inner));
        }
        last.unwrap()
    }
}
