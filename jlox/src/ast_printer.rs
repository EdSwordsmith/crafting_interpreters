use crate::ast::{Expr, ExprVisitor, Object};

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&mut self, name: impl AsRef<str>, exprs: &[&Expr]) -> String {
        let mut s = String::new();

        s.push('(');
        s.push_str(name.as_ref());

        exprs
            .iter()
            .map(|expr| self.visit_expr(expr))
            .for_each(|res| {
                s.push_str(format!(" {}", res).as_str());
            });

        s.push(')');
        s
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_expr(&mut self, expression: &Expr) -> String {
        match expression {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(&operator.lexeme, &[left, right]),
            Expr::Grouping { expression } => self.parenthesize("group", &[expression]),
            Expr::Literal { value } => match value {
                Object::Number(n) => n.to_string(),
                Object::Bool(b) => b.to_string(),
                Object::String(s) => s.clone(),
                Object::Nil => String::from("nil"),
            },
            Expr::Unary { operator, right } => self.parenthesize(&operator.lexeme, &[right]),
            Expr::Variable { name } => format!("(var {})", name.lexeme),
            Expr::Assignment { .. } => todo!(),
        }
    }
}
