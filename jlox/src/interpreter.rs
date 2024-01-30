use std::collections::HashMap;

use crate::{
    ast::{Expr, ExprVisitor, Object, Stmt, StmtVisitor},
    errors::RuntimeError,
    scanner::{Token, TokenType},
};

#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
}

fn runtime_error(token: &Token, message: &str) -> RuntimeError {
    RuntimeError {
        line: token.line,
        message: message.to_string(),
    }
}

impl Interpreter {
    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for statement in statements {
            self.visit_stmt(statement)?;
        }

        Ok(())
    }
}

impl ExprVisitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping { expression } => self.visit_expr(expression),

            Expr::Unary { operator, right } => {
                let result = self.visit_expr(right)?;

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
                let left = self.visit_expr(left)?;
                let right = self.visit_expr(right)?;

                match (&operator.token_type, left, right) {
                    (TokenType::Minus, Object::Number(l), Object::Number(r)) => {
                        Ok(Object::Number(l - r))
                    }
                    (TokenType::Minus, _, _) => {
                        Err(runtime_error(operator, "Operands must be numbers."))
                    }

                    (TokenType::Slash, Object::Number(l), Object::Number(r)) => {
                        Ok(Object::Number(l / r))
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

            Expr::Variable { name } => self.environment.get(name),

            Expr::Assignment { name, value } => {
                let value = self.visit_expr(value)?;
                self.environment.assign(name, value)
            }
        }
    }
}

impl StmtVisitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_stmt(&mut self, statement: &Stmt) -> Result<(), RuntimeError> {
        match statement {
            Stmt::Expression { expression } => {
                self.visit_expr(expression)?;
                Ok(())
            }

            Stmt::Print { expression } => {
                let value = self.visit_expr(expression)?;
                println!("{value}");
                Ok(())
            }

            Stmt::Var { name, initializer } => {
                let value = if let Some(expr) = initializer {
                    Some(self.visit_expr(&expr)?)
                } else {
                    None
                };
                self.environment.define(name.lexeme.clone(), value);
                Ok(())
            }

            Stmt::Block { statements } => {
                self.environment.push();
                for statement in statements.iter() {
                    self.visit_stmt(statement)?;
                }
                self.environment.pop();
                Ok(())
            }
        }
    }
}

struct Environment {
    scopes: Vec<HashMap<String, Option<Object>>>,
}

impl Default for Environment {
    fn default() -> Self {
        let global_scope = HashMap::new();
        Self {
            scopes: vec![global_scope],
        }
    }
}

impl Environment {
    fn define(&mut self, name: String, value: Option<Object>) {
        if let Some(values) = self.scopes.last_mut() {
            values.insert(name, value);
        }
    }

    fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        let error_msg = format!("Undefined variable '{}'.", name.lexeme);
        let unit_msg = format!("Uninitialized variable '{}'.", name.lexeme);

        let res = self
            .scopes
            .iter()
            .filter_map(|values| values.get(&name.lexeme))
            .last();

        if let Some(res) = res {
            if let Some(res) = res {
                Ok(res.clone())
            } else {
                Err(runtime_error(name, &unit_msg))
            }
        } else {
            Err(runtime_error(name, &error_msg))
        }
    }

    fn assign(&mut self, name: &Token, value: Object) -> Result<Object, RuntimeError> {
        self.get(name)?;
        for values in self.scopes.iter_mut().rev() {
            if values.contains_key(&name.lexeme) {
                values.insert(name.lexeme.clone(), Some(value.clone()));
            }
        }

        Ok(value)
    }

    fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.scopes.pop();
    }
}
