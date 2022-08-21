use crate::ast::{Expr, ExprVisitor, Object};
use crate::rpn_printer::RpnPrinter;
use crate::token::{Token, TokenType};

mod ast;
mod rpn_printer;
mod token;

fn main() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: Object::Number(1.0),
                }),
                operator: Token::new(TokenType::Plus, "+".into(), 1),
                right: Box::new(Expr::Literal {
                    value: Object::Number(2.0),
                }),
            }),
        }),
        operator: Token::new(TokenType::Star, "*".into(), 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: Object::Number(4.0),
                }),
                operator: Token::new(TokenType::Minus, "-".into(), 1),
                right: Box::new(Expr::Literal {
                    value: Object::Number(3.0),
                }),
            }),
        }),
    };

    println!("{}", RpnPrinter.visit(&expr));
}
