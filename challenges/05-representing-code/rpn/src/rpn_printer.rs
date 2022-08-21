use crate::ast::{Expr, ExprVisitor, Object};
use crate::TokenType;

pub struct RpnPrinter;

impl RpnPrinter {
    fn parenthesize(&mut self, name: impl AsRef<str>, exprs: &[&Expr]) -> String {
        let mut s = String::new();

        exprs.iter().map(|expr| self.visit(expr)).for_each(|res| {
            s.push_str(format!("{} ", res).as_str());
        });

        s.push_str(name.as_ref());

        s
    }
}

impl ExprVisitor<String> for RpnPrinter {
    fn visit(&mut self, expression: &Expr) -> String {
        match expression {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(&operator.lexeme, &[left, right]),
            Expr::Grouping { expression } => self.visit(expression),
            Expr::Literal { value } => match value {
                Object::Number(n) => n.to_string(),
                Object::String(s) => s.clone(),
                Object::Nil => String::from("nil"),
            },
            Expr::Unary { operator, right } => match operator.token_type {
                TokenType::Minus => self.parenthesize("~", &[right]),
                _ => self.visit(right),
            },
        }
    }
}
