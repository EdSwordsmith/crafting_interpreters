use crate::{
    ast::{Expr, ExprVisitor, Object},
    errors::RuntimeError,
    scanner::{Token, TokenType},
};

pub struct Interpreter {}

fn runtime_error(token: &Token, message: &str) -> RuntimeError {
    RuntimeError {
        line: token.line,
        message: message.to_string(),
    }
}

impl Interpreter {
    pub fn interpret(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        let value = self.visit(expr)?;
        println!("{value}");
        Ok(())
    }
}

impl ExprVisitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping { expression } => self.visit(expression),

            Expr::Unary { operator, right } => {
                let result = self.visit(right)?;

                match (&operator.token_type, result) {
                    (TokenType::Minus, Object::Number(value)) => Ok(Object::Number(-value)),
                    (TokenType::Minus, _) => {
                        Err(runtime_error(operator, "Operand must be a number."))
                    }
                    (TokenType::Bang, result) => Ok(Object::Bool(!result.truthy())),
                    _ => unreachable!(),
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.visit(left)?;
                let right = self.visit(right)?;

                match (&operator.token_type, left, right) {
                    (TokenType::Minus, Object::Number(l), Object::Number(r)) => {
                        Ok(Object::Number(l - r))
                    }
                    (TokenType::Minus, _, _) => {
                        Err(runtime_error(operator, "Operands must be numbers."))
                    }

                    (TokenType::Slash, Object::Number(l), Object::Number(r)) => {
                        if r != 0.0 {
                            Ok(Object::Number(l / r))
                        } else {
                            Err(runtime_error(operator, "Cannot divide by zero."))
                        }
                    }
                    (TokenType::Slash, _, _) => {
                        Err(runtime_error(operator, "Operands must be numbers."))
                    }

                    (TokenType::Star, Object::Number(l), Object::Number(r)) => {
                        Ok(Object::Number(l * r))
                    }
                    (TokenType::Star, _, _) => {
                        Err(runtime_error(operator, "Operands must be numbers."))
                    }

                    (TokenType::Plus, Object::Number(l), Object::Number(r)) => {
                        Ok(Object::Number(l + r))
                    }
                    (TokenType::Plus, Object::String(l), Object::String(r)) => {
                        Ok(Object::String(l + r.as_str()))
                    }
                    (TokenType::Plus, _, _) => Err(runtime_error(
                        operator,
                        "Operands must be two numbers or two strings.",
                    )),

                    (TokenType::Greater, Object::Number(l), Object::Number(r)) => {
                        Ok(Object::Bool(l > r))
                    }
                    (TokenType::Greater, _, _) => {
                        Err(runtime_error(operator, "Operands must be numbers."))
                    }

                    (TokenType::GreaterEqual, Object::Number(l), Object::Number(r)) => {
                        Ok(Object::Bool(l >= r))
                    }
                    (TokenType::GreaterEqual, _, _) => {
                        Err(runtime_error(operator, "Operands must be numbers."))
                    }

                    (TokenType::Less, Object::Number(l), Object::Number(r)) => {
                        Ok(Object::Bool(l < r))
                    }
                    (TokenType::Less, _, _) => {
                        Err(runtime_error(operator, "Operands must be numbers."))
                    }

                    (TokenType::LessEqual, Object::Number(l), Object::Number(r)) => {
                        Ok(Object::Bool(l <= r))
                    }
                    (TokenType::LessEqual, _, _) => {
                        Err(runtime_error(operator, "Operands must be numbers."))
                    }

                    (TokenType::BangEqual, l, r) => Ok(Object::Bool(l != r)),

                    (TokenType::EqualEqual, l, r) => Ok(Object::Bool(l == r)),

                    _ => unreachable!(),
                }
            }
        }
    }
}
