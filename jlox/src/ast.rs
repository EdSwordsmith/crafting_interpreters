use std::fmt::Display;

use crate::scanner::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
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
        };

        write!(f, "{res}")
    }
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Object,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub trait ExprVisitor<T> {
    fn visit(&mut self, expression: &Expr) -> T;
}
