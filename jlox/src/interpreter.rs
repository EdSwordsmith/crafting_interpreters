use std::{cell::RefCell, collections::HashMap, rc::Rc};

use chrono::Utc;

use crate::{
    ast::{Expr, ExprVisitor, Stmt, StmtVisitor},
    errors::RuntimeError,
    scanner::{Token, TokenType},
    values::{
        Callable::{self, NativeFn},
        Object,
    },
};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
    pub locals: HashMap<Expr, usize>,
}

fn runtime_error(token: &Token, message: &str) -> RuntimeError {
    RuntimeError {
        line: token.line,
        message: message.to_string(),
    }
}

macro_rules! native_fn {
    ($env:expr, $name:expr, $arity:expr, $func:expr) => {
        $env.borrow_mut().define(
            $name.into(),
            Object::Callable(NativeFn($name.into(), $arity, $func)),
        );
    };
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Environment::new();
        native_fn!(environment, "clock", 0, |_, _| {
            Ok(Object::Number(
                Utc::now().timestamp_millis() as f64 / 1000.0,
            ))
        });

        Self {
            environment: environment.clone(),
            globals: environment,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for statement in statements {
            self.visit_stmt(statement)?;
        }

        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Rc<RefCell<Environment>>,
    ) -> Result<Option<Object>, RuntimeError> {
        let previous = self.environment.clone();
        self.environment = environment;

        let mut return_value = None;
        for statement in statements.iter() {
            if let Some(value) = self.visit_stmt(statement)? {
                return_value = Some(value);
                break;
            }
        }

        self.environment = previous;
        Ok(return_value)
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }

    fn lookup_variable(&self, name: &Token, expr: &Expr) -> Result<Object, RuntimeError> {
        if let Some(distance) = self.locals.get(expr) {
            self.environment
                .borrow()
                .get_at(*distance, name.lexeme.clone())
        } else {
            self.globals.borrow().get(name)
        }
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

            Expr::Variable { name } => self.lookup_variable(name, expr),

            Expr::Assignment { name, value } => {
                let value = self.visit_expr(value)?;

                if let Some(distance) = self.locals.get(expr) {
                    self.environment
                        .borrow_mut()
                        .assign_at(*distance, name, value)
                } else {
                    self.globals.borrow_mut().assign(name, value)
                }
            }

            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expr(left)?;
                match (&operator.token_type, left) {
                    (TokenType::Or, left) if left.truthy() => Ok(left),
                    (TokenType::And, left) if !left.truthy() => Ok(left),
                    _ => self.visit_expr(right),
                }
            }

            Expr::Call {
                callee,
                arguments,
                paren,
            } => {
                let callee = self.visit_expr(callee)?;
                let mut args = Vec::new();
                for argument in arguments.iter() {
                    args.push(self.visit_expr(argument)?);
                }

                if let Object::Callable(callee) = callee {
                    if callee.arity() != args.len() {
                        Err(runtime_error(
                            paren,
                            &format!(
                                "Expected {} arguments but got instead {}.",
                                callee.arity(),
                                args.len()
                            ),
                        ))
                    } else {
                        callee.call(self, args)
                    }
                } else {
                    Err(runtime_error(paren, "Can only call functions and classes."))
                }
            }
        }
    }
}

impl StmtVisitor<Result<Option<Object>, RuntimeError>> for Interpreter {
    fn visit_stmt(&mut self, statement: &Stmt) -> Result<Option<Object>, RuntimeError> {
        match statement {
            Stmt::Expression { expression } => {
                self.visit_expr(expression)?;
                Ok(None)
            }

            Stmt::Print { expression } => {
                let value = self.visit_expr(expression)?;
                println!("{value}");
                Ok(None)
            }

            Stmt::Var { name, initializer } => {
                let value = self.visit_expr(initializer)?;
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value);
                Ok(None)
            }

            Stmt::Block { statements } => self.execute_block(
                statements,
                Environment::with_enclosing(self.environment.clone()),
            ),

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_value = self.visit_expr(condition)?;
                let res = if condition_value.truthy() {
                    self.visit_stmt(then_branch)?
                } else if let Some(else_branch) = else_branch {
                    self.visit_stmt(else_branch)?
                } else {
                    None
                };

                Ok(res)
            }

            Stmt::While { condition, body } => {
                while self.visit_expr(condition)?.truthy() {
                    if let Some(value) = self.visit_stmt(body)? {
                        return Ok(Some(value));
                    }
                }
                Ok(None)
            }

            Stmt::Function { name, .. } => {
                let rc = self.environment.clone();
                let function = Callable::LoxFn(Box::new(statement.clone()), rc);
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Object::Callable(function));
                Ok(None)
            }

            Stmt::Return { expression, .. } => Ok(Some(self.visit_expr(expression)?)),
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enclosing: None,
            values: HashMap::new(),
        }))
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }))
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        let error_msg = format!("Undefined variable '{}'.", name.lexeme);
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(runtime_error(name, &error_msg))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<Object, RuntimeError> {
        let error_msg = format!("Undefined variable '{}'.", name.lexeme);
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            Ok(value)
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(runtime_error(name, &error_msg))
        }
    }

    fn ancestor(&self, distance: usize) -> Option<Rc<RefCell<Environment>>> {
        let mut environment = Some(Rc::new(RefCell::new(self.clone())));
        for _ in 0..distance {
            if let Some(env) = environment {
                environment = env.borrow().enclosing.clone();
            }
        }
        environment
    }

    pub fn get_at(&self, distance: usize, name: String) -> Result<Object, RuntimeError> {
        if let Some(ancestor) = self.ancestor(distance) {
            Ok(ancestor.borrow().values.get(&name).unwrap().clone())
        } else {
            unreachable!()
        }
    }

    pub fn assign_at(
        &self,
        distance: usize,
        name: &Token,
        value: Object,
    ) -> Result<Object, RuntimeError> {
        if let Some(ancestor) = self.ancestor(distance) {
            ancestor
                .borrow_mut()
                .values
                .insert(name.lexeme.clone(), value.clone());
            Ok(value)
        } else {
            unreachable!()
        }
    }
}
