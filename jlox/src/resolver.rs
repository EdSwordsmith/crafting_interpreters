use std::collections::HashMap;

use crate::{
    ast::{Expr, ExprVisitor, Stmt, StmtVisitor},
    errors::{error, LoxError},
    interpreter::Interpreter,
    parser::parser_error,
    scanner::Token,
    values::LoxPrimitive,
};

#[derive(Clone, Copy)]
enum ClassType {
    None,
    Class,
    Subclass,
}

#[derive(Clone, Copy)]
enum FunctionType {
    None,
    Function,
    Method,
    Initializer,
}

pub struct Resolver<'a> {
    scopes: Vec<HashMap<String, bool>>,
    interpreter: &'a mut Interpreter,
    current_function: FunctionType,
    current_class: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            scopes: Vec::new(),
            interpreter,
            current_function: FunctionType::None,
            current_class: ClassType::None,
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> Result<(), LoxError> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.insert(name.lexeme.clone(), false).is_some() {
                return Err(parser_error(
                    name,
                    "Already a variable with this name in this scope.",
                ));
            }
        }

        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    pub fn resolve(&mut self, stmts: &[Stmt]) -> Result<(), LoxError> {
        for stmt in stmts.iter() {
            self.visit_stmt(stmt)?;
        }

        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        let index = self
            .scopes
            .iter()
            .rev()
            .position(|scope| scope.contains_key(&name.lexeme));

        if let Some(index) = index {
            self.interpreter.resolve(expr, index);
        }
    }

    fn resolve_function(
        &mut self,
        function: &Stmt,
        function_type: FunctionType,
    ) -> Result<(), LoxError> {
        if let Stmt::Function { params, body, .. } = function {
            let enclosing = self.current_function;
            self.current_function = function_type;
            self.begin_scope();
            for param in params.iter() {
                self.declare(param)?;
                self.define(param);
            }
            self.resolve(body)?;
            self.end_scope();
            self.current_function = enclosing;
        }

        Ok(())
    }
}

impl<'a> ExprVisitor<Result<(), LoxError>> for Resolver<'a> {
    fn visit_expr(&mut self, expression: &crate::ast::Expr) -> Result<(), LoxError> {
        match expression {
            Expr::Variable { name } => {
                if let Some(scope) = self.scopes.last() {
                    if !scope.get(&name.lexeme).unwrap_or(&true) {
                        return Err(error(
                            name.line,
                            "Can't read local variable in its own initializer.",
                        ));
                    }
                }

                self.resolve_local(expression, name);

                Ok(())
            }

            Expr::Assignment { name, value } => {
                self.visit_expr(value)?;
                self.resolve_local(expression, name);
                Ok(())
            }

            Expr::Binary { left, right, .. } | Expr::Logical { left, right, .. } => {
                self.visit_expr(left)?;
                self.visit_expr(right)
            }

            Expr::Call {
                callee, arguments, ..
            } => {
                self.visit_expr(callee)?;
                for argument in arguments.iter() {
                    self.visit_expr(argument)?;
                }
                Ok(())
            }

            Expr::Grouping { expression }
            | Expr::Unary {
                right: expression, ..
            } => self.visit_expr(expression),

            Expr::Literal { .. } => Ok(()),

            Expr::Get { object, .. } => self.visit_expr(object),

            Expr::Set { object, value, .. } => {
                self.visit_expr(value)?;
                self.visit_expr(object)
            }

            Expr::This { keyword } => {
                if let ClassType::Class = self.current_class {
                    self.resolve_local(expression, keyword);
                    Ok(())
                } else {
                    Err(parser_error(
                        keyword,
                        "Can't use 'this' outside of a class.",
                    ))
                }
            }
        }
    }
}

impl<'a> StmtVisitor<Result<(), LoxError>> for Resolver<'a> {
    fn visit_stmt(&mut self, statement: &Stmt) -> Result<(), LoxError> {
        match statement {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();

                Ok(())
            }

            Stmt::Var { name, initializer } => {
                self.declare(name)?;
                self.visit_expr(initializer)?;
                self.define(name);

                Ok(())
            }

            Stmt::Function { name, .. } => {
                self.declare(name)?;
                self.define(name);
                self.resolve_function(statement, FunctionType::Function)
            }

            Stmt::Expression { expression } | Stmt::Print { expression } => {
                self.visit_expr(expression)
            }

            Stmt::Return {
                keyword,
                expression,
            } => {
                if let FunctionType::None = self.current_function {
                    Err(parser_error(keyword, "Can't return from top-level code."))
                } else if let FunctionType::Initializer = self.current_function {
                    if let Expr::Literal { value } = *expression.clone() {
                        if let Some(LoxPrimitive::Nil) = value.0.borrow().primitive() {
                            Ok(())
                        } else {
                            Err(parser_error(
                                keyword,
                                "Can't return a value from an initializer.",
                            ))
                        }
                    } else {
                        Err(parser_error(
                            keyword,
                            "Can't return a value from an initializer.",
                        ))
                    }
                } else {
                    self.visit_expr(expression)
                }
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expr(condition)?;
                self.visit_stmt(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.visit_stmt(else_branch)?;
                }
                Ok(())
            }

            Stmt::While { condition, body } => {
                self.visit_expr(condition)?;
                self.visit_stmt(body)
            }

            Stmt::Class {
                name,
                methods,
                superclass,
            } => {
                let enclosing = self.current_class;
                self.current_class = ClassType::Class;

                self.declare(name)?;
                self.define(name);

                if let Some(superclass) = superclass {
                    self.current_class = ClassType::Subclass;
                    if let Expr::Variable {
                        name: superclass_name,
                    } = *superclass.clone()
                    {
                        if name.lexeme == superclass_name.lexeme {
                            return Err(parser_error(
                                &superclass_name,
                                "A class can't inherit from itself.",
                            ));
                        }
                    }

                    self.visit_expr(superclass)?;
                }

                if superclass.is_some() {
                    self.begin_scope();
                    if let Some(scope) = self.scopes.last_mut() {
                        // scope.insert("super".into(), true);
                    }
                }

                self.begin_scope();
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert("this".into(), true);
                    scope.insert("inner".into(), true);
                }

                for method in methods.iter() {
                    let declaration = if let Stmt::Function { name, .. } = method {
                        if name.lexeme == "init" {
                            FunctionType::Initializer
                        } else {
                            FunctionType::Method
                        }
                    } else {
                        unreachable!()
                    };

                    self.resolve_function(method, declaration)?;
                }

                self.end_scope();

                if superclass.is_some() {
                    self.end_scope();
                }

                self.current_class = enclosing;

                Ok(())
            }
        }
    }
}
