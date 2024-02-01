use crate::ast::{Expr, ExprVisitor};

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
            Expr::Literal { value } => format!("{value}"),
            Expr::Unary { operator, right } => self.parenthesize(&operator.lexeme, &[right]),
            Expr::Variable { name } => format!("(var {})", name.lexeme),
            Expr::Assignment { .. } => todo!(),
            Expr::Logical { .. } => todo!(),
            Expr::Call { .. } => todo!(),
            Expr::Function { .. } => todo!(),
        }
    }
}
