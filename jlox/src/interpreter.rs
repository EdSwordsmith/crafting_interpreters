use std::{cell::RefCell, collections::HashMap, rc::Rc};

use chrono::Utc;

use crate::{
    ast::{Expr, ExprVisitor, Stmt, StmtVisitor},
    errors::RuntimeError,
    scanner::{Token, TokenType},
    values::{boolean, lox_class, lox_fn, native_fn, nil, number, LoxObj, LoxProperty},
};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
    pub locals: HashMap<Expr, usize>,
}

pub fn runtime_error(token: &Token, message: &str) -> RuntimeError {
    RuntimeError {
        line: token.line,
        message: message.to_string(),
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Environment::new();
        environment.borrow_mut().define(
            "clock".into(),
            native_fn(0, |_, _| {
                Ok(number(Utc::now().timestamp_millis() as f64 / 1000.0))
            }),
        );

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
    ) -> Result<Option<LoxObj>, RuntimeError> {
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

    fn lookup_variable(&self, name: &Token, expr: &Expr) -> Result<LoxObj, RuntimeError> {
        if let Some(distance) = self.locals.get(expr) {
            self.environment
                .borrow()
                .get_at(*distance, name.lexeme.clone())
        } else {
            self.globals.borrow().get(name)
        }
    }
}

impl ExprVisitor<Result<LoxObj, RuntimeError>> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Result<LoxObj, RuntimeError> {
        match expr {
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping { expression } => self.visit_expr(expression),

            Expr::Unary { operator, right } => {
                let result = self.visit_expr(right)?;
                let neg_err = runtime_error(operator, "Operand must be a number.");

                match operator.token_type {
                    TokenType::Minus => (-result).ok_or(neg_err),
                    TokenType::Bang => Ok(boolean(!result.0.borrow().is_truthy())),
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
                let ord = left.partial_cmp(&right);

                let numbers_err = runtime_error(operator, "Operands must be numbers.");
                let sum_err =
                    runtime_error(operator, "Operands must be two numbers or two strings.");

                match (&operator.token_type, ord) {
                    (TokenType::Minus, _) => (left - right).ok_or(numbers_err),
                    (TokenType::Slash, _) => (left / right).ok_or(numbers_err),
                    (TokenType::Star, _) => (left * right).ok_or(numbers_err),
                    (TokenType::Plus, _) => (left + right).ok_or(sum_err),

                    (TokenType::Greater, None)
                    | (TokenType::GreaterEqual, None)
                    | (TokenType::Less, None)
                    | (TokenType::LessEqual, None) => Err(numbers_err),

                    (TokenType::Greater, _) => Ok(boolean(left > right)),
                    (TokenType::GreaterEqual, _) => Ok(boolean(left >= right)),
                    (TokenType::Less, _) => Ok(boolean(left < right)),
                    (TokenType::LessEqual, _) => Ok(boolean(left <= right)),
                    (TokenType::BangEqual, _) => Ok(left.is_diff(&right)),
                    (TokenType::EqualEqual, _) => Ok(left.is_equal(&right)),

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
                    (TokenType::Or, left) if left.0.borrow().is_truthy() => Ok(left),
                    (TokenType::And, left) if !left.0.borrow().is_truthy() => Ok(left),
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

                if let Some(callee) = callee.callable() {
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
                        callee.call(self, &args)
                    }
                } else {
                    Err(runtime_error(paren, "Can only call functions and classes."))
                }
            }

            Expr::Get { object, name } => {
                let object = self.visit_expr(object)?;
                let res = object.0.borrow().get_property(name);
                match res {
                    LoxProperty::Invalid => {
                        Err(runtime_error(name, "Only instances have properties."))
                    }

                    LoxProperty::Undef => Err(runtime_error(
                        name,
                        &format!("Undefined property '{}'.", name.lexeme),
                    )),

                    LoxProperty::Field(obj) => Ok(obj),

                    LoxProperty::Method(obj) => Ok(obj.bind(object)),
                }
            }

            Expr::Set {
                object,
                name,
                value,
            } => {
                let mut object = self.visit_expr(object)?;
                let value = self.visit_expr(value)?;
                object
                    .set_property(name, &value)
                    .ok_or(runtime_error(name, "Only instances have fields."))
            }

            Expr::This { keyword } => self.lookup_variable(keyword, expr),
        }
    }
}

impl StmtVisitor<Result<Option<LoxObj>, RuntimeError>> for Interpreter {
    fn visit_stmt(&mut self, statement: &Stmt) -> Result<Option<LoxObj>, RuntimeError> {
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
                let res = if condition_value.0.borrow().is_truthy() {
                    self.visit_stmt(then_branch)?
                } else if let Some(else_branch) = else_branch {
                    self.visit_stmt(else_branch)?
                } else {
                    None
                };

                Ok(res)
            }

            Stmt::While { condition, body } => {
                while self.visit_expr(condition)?.0.borrow().is_truthy() {
                    if let Some(value) = self.visit_stmt(body)? {
                        return Ok(Some(value));
                    }
                }
                Ok(None)
            }

            Stmt::Function { name, .. } => {
                let rc = self.environment.clone();
                let function = lox_fn(Box::new(statement.clone()), rc, false);
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), function);
                Ok(None)
            }

            Stmt::Return { expression, .. } => Ok(Some(self.visit_expr(expression)?)),

            Stmt::Class {
                name,
                methods,
                class_methods,
            } => {
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), nil());

                let mut methods_fn = HashMap::new();
                for method in methods.iter() {
                    if let Stmt::Function { name, .. } = method {
                        let method = lox_fn(
                            Box::new(method.clone()),
                            self.environment.clone(),
                            name.lexeme == "init",
                        );
                        methods_fn.insert(name.lexeme.clone(), method);
                    }
                }

                let mut class_methods_fn = HashMap::new();
                for method in class_methods.iter() {
                    if let Stmt::Function { name, .. } = method {
                        let method =
                            lox_fn(Box::new(method.clone()), self.environment.clone(), false);
                        class_methods_fn.insert(name.lexeme.clone(), method);
                    }
                }

                let class = lox_class(name.lexeme.clone(), methods_fn, class_methods_fn.clone());

                self.environment.borrow_mut().assign(name, class)?;
                Ok(None)
            }
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, LoxObj>,
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

    pub fn define(&mut self, name: String, value: LoxObj) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<LoxObj, RuntimeError> {
        let error_msg = format!("Undefined variable '{}'.", name.lexeme);
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(runtime_error(name, &error_msg))
        }
    }

    pub fn assign(&mut self, name: &Token, value: LoxObj) -> Result<LoxObj, RuntimeError> {
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

    pub fn get_at(&self, distance: usize, name: String) -> Result<LoxObj, RuntimeError> {
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
        value: LoxObj,
    ) -> Result<LoxObj, RuntimeError> {
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
