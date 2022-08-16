use crate::scanner::Token;

pub enum Object {
    Number(f64),
    String(String),
    Nil
}

pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr>, },
    Grouping { expression: Box<Expr>, },
    Literal { value: Object },
    Unary { operator: Token, right: Box<Expr>, },
}

pub trait ExprVisitor<T> {
    fn visit(&mut self, expression: &Expr) -> T;
}
