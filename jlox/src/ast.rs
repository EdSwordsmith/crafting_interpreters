use crate::scanner::Token;

pub enum Object {
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
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
    Ternary {
        condition: Box<Expr>,
        if_true: Box<Expr>,
        if_false: Box<Expr>,
    },
}

pub trait ExprVisitor<T> {
    fn visit(&mut self, expression: &Expr) -> T;
}
