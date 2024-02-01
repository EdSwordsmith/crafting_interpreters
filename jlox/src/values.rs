use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    hash,
    rc::Rc,
};

use crate::{
    ast::Stmt,
    errors::RuntimeError,
    interpreter::{Environment, Interpreter},
};

#[derive(Clone, PartialEq, Debug)]
pub enum Object {
    Number(f64),
    Bool(bool),
    String(String),
    Callable(Callable),
    Nil,
}

impl Eq for Object {}
impl hash::Hash for Object {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Object {
    pub fn truthy(&self) -> bool {
        match self {
            Object::Nil => false,
            Object::Bool(value) => *value,
            _ => true,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Object::Number(n) => n.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::String(s) => s.clone(),
            Object::Nil => "nil".into(),
            Object::Callable(c) => c.to_string(),
        };

        write!(f, "{res}")
    }
}

#[derive(Clone)]
pub enum Callable {
    NativeFn(
        String,
        usize,
        fn(&mut Interpreter, Vec<Object>) -> Result<Object, RuntimeError>,
    ),
    LoxFn(Box<Stmt>, Rc<RefCell<Environment>>),
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NativeFn(name_l, _, _), Self::NativeFn(name_r, _, _)) => name_l == name_r,
            (Self::LoxFn(l0, _), Self::LoxFn(r0, _)) => *l0 == *r0,
            _ => false,
        }
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Callable::NativeFn(_, _, _) => write!(f, "<native fn>"),
            Callable::LoxFn(fun, _) => match *fun.clone() {
                Stmt::Function { name, .. } => write!(f, "<fn {}>", name.lexeme),
                _ => unreachable!(),
            },
        }
    }
}

impl Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Callable {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        match self {
            Callable::NativeFn(_, _, function) => function(interpreter, arguments),
            Callable::LoxFn(fun, closure) => match *fun.clone() {
                Stmt::Function { params, body, .. } => {
                    let environment = Environment::with_enclosing(closure.clone());
                    for (param, value) in params.iter().zip(arguments.iter()) {
                        environment
                            .borrow_mut()
                            .define(param.lexeme.clone(), value.clone());
                    }

                    let res = interpreter.execute_block(&body, environment)?;
                    Ok(res.unwrap_or(Object::Nil))
                }
                _ => unreachable!(),
            },
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Callable::NativeFn(_, value, _) => *value,
            Callable::LoxFn(fun, _) => match *fun.clone() {
                Stmt::Function { params, .. } => params.len(),
                _ => unreachable!(),
            },
        }
    }
}
