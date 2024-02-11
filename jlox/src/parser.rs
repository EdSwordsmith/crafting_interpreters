use std::matches;

use crate::{
    ast::{Expr, Stmt},
    errors::{error_with_location, Errors, LoxError},
    scanner::{Token, TokenType},
    values::{boolean, nil, number, string},
};

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, Errors> {
    let mut state = State::new(tokens);
    let mut statements = Vec::new();
    let mut errors = Vec::new();

    while !state.is_at_end() {
        match state.declaration() {
            Ok(stmt) => statements.push(stmt),
            Err(mut errs) => errors.append(&mut errs),
        }
    }

    if errors.is_empty() {
        Ok(statements)
    } else {
        Err(Errors::Parsing(errors))
    }
}

pub fn parser_error(token: &Token, message: &str) -> LoxError {
    match token.token_type {
        TokenType::Eof => error_with_location(token.line, &" at end", message),
        _ => error_with_location(
            token.line,
            &(String::from(" at '") + &token.lexeme + "'"),
            message,
        ),
    }
}

struct State {
    tokens: Vec<Token>,
    current: usize,
}

impl State {
    fn new(tokens: Vec<Token>) -> State {
        State { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        let matched = types.iter().any(|token_type| self.check(token_type));
        if matched {
            self.advance();
        }

        matched
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, LoxError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(parser_error(self.peek(), message))
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let TokenType::Semicolon = self.previous().token_type {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }

    /* Grammar rules functions */
    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.matches(&[TokenType::False]) {
            Ok(Expr::Literal {
                value: boolean(false),
            })
        } else if self.matches(&[TokenType::True]) {
            Ok(Expr::Literal {
                value: boolean(true),
            })
        } else if self.matches(&[TokenType::Nil]) {
            Ok(Expr::Literal { value: nil() })
        } else if self.matches(&[TokenType::Number(0.0), TokenType::String("".into())]) {
            let obj = match &self.previous().token_type {
                TokenType::Number(value) => number(*value),
                TokenType::String(value) => string(value.clone()),
                _ => unreachable!(),
            };
            Ok(Expr::Literal { value: obj })
        } else if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else if self.matches(&[TokenType::Identifier]) {
            Ok(Expr::Variable {
                name: self.previous().clone(),
            })
        } else if self.matches(&[TokenType::This]) {
            Ok(Expr::This {
                keyword: self.previous().clone(),
            })
        } else {
            Err(parser_error(self.peek(), "Expect expression."))
        }
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LoxError> {
        let mut arguments = Vec::new();
        if !self.check(&TokenType::RightParen) {
            arguments.push(self.expression()?);
            while self.matches(&[TokenType::Comma]) {
                if arguments.len() >= 255 {
                    return Err(parser_error(
                        self.peek(),
                        "Can't have more than 255 arguments.",
                    ));
                }

                arguments.push(self.expression()?);
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren: paren.clone(),
            arguments,
        })
    }

    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.matches(&[TokenType::Dot]) {
                let name = self
                    .consume(&TokenType::Identifier, "Expect property name after '.'.")?
                    .clone();
                expr = Expr::Get {
                    object: Box::new(expr),
                    name,
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            Ok(Expr::Unary {
                operator: self.previous().clone(),
                right: Box::new(self.unary()?),
            })
        } else {
            self.call()
        }
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Star, TokenType::Slash]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: self.previous().clone(),
                right: Box::new(self.unary()?),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Plus, TokenType::Minus]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: self.previous().clone(),
                right: Box::new(self.factor()?),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: self.previous().clone(),
                right: Box::new(self.term()?),
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: self.previous().clone(),
                right: Box::new(self.comparison()?),
            }
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            expr = Expr::Logical {
                left: Box::new(expr),
                operator: self.previous().clone(),
                right: Box::new(self.equality()?),
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            expr = Expr::Logical {
                left: Box::new(expr),
                operator: self.previous().clone(),
                right: Box::new(self.and()?),
            }
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.or()?;

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable { name } = expr {
                Ok(Expr::Assignment {
                    name,
                    value: Box::new(value),
                })
            } else if let Expr::Get { object, name } = expr {
                Ok(Expr::Set {
                    object,
                    name,
                    value: Box::new(value),
                })
            } else {
                Err(parser_error(&equals, "Invalid assignment target."))
            }
        } else {
            Ok(expr)
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn expression_statement(&mut self) -> Result<Stmt, Vec<LoxError>> {
        let expression = self.expression().map_err(|error| vec![error])?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")
            .map_err(|error| vec![error])?;
        Ok(Stmt::Expression {
            expression: Box::new(expression),
        })
    }

    fn print_statement(&mut self) -> Result<Stmt, Vec<LoxError>> {
        let value = self.expression().map_err(|error| vec![error])?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")
            .map_err(|error| vec![error])?;
        Ok(Stmt::Print {
            expression: Box::new(value),
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Vec<LoxError>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(mut errs) => errors.append(&mut errs),
            }
        }

        match self.consume(&TokenType::RightBrace, "Expect '}' after block.") {
            Ok(_) => {}
            Err(error) => errors.push(error),
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, Vec<LoxError>> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.")
            .map_err(|err| vec![err])?;
        let condition = Box::new(self.expression().map_err(|err| vec![err])?);
        self.consume(&TokenType::RightParen, "Expect ')' after if condition.")
            .map_err(|err| vec![err])?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, Vec<LoxError>> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.")
            .map_err(|err| vec![err])?;
        let condition = Box::new(self.expression().map_err(|err| vec![err])?);
        self.consume(&TokenType::RightParen, "Expect ')' after condition.")
            .map_err(|err| vec![err])?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Stmt, Vec<LoxError>> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.")
            .map_err(|err| vec![err])?;
        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.check(&TokenType::Semicolon) {
            Box::new(Expr::Literal {
                value: boolean(true),
            })
        } else {
            Box::new(self.expression().map_err(|err| vec![err])?)
        };
        self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")
            .map_err(|err| vec![err])?;

        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(Box::new(self.expression().map_err(|err| vec![err])?))
        };
        self.consume(&TokenType::RightParen, "Expect ')' after condition.")
            .map_err(|err| vec![err])?;

        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = Stmt::Block {
                statements: vec![
                    body,
                    Stmt::Expression {
                        expression: increment,
                    },
                ],
            }
        }

        body = Stmt::While {
            condition,
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Stmt::Block {
                statements: vec![initializer, body],
            }
        }

        Ok(body)
    }

    fn return_statement(&mut self) -> Result<Stmt, Vec<LoxError>> {
        let keyword = self.previous().clone();
        let value = if self.check(&TokenType::Semicolon) {
            Expr::Literal { value: nil() }
        } else {
            self.expression().map_err(|err| vec![err])?
        };

        self.consume(&TokenType::Semicolon, "Expect ';' after return value.")
            .map_err(|err| vec![err])?;

        Ok(Stmt::Return {
            keyword,
            expression: Box::new(value),
        })
    }

    fn statement(&mut self) -> Result<Stmt, Vec<LoxError>> {
        if self.matches(&[TokenType::For]) {
            self.for_statement()
        } else if self.matches(&[TokenType::If]) {
            self.if_statement()
        } else if self.matches(&[TokenType::Print]) {
            self.print_statement()
        } else if self.matches(&[TokenType::Return]) {
            self.return_statement()
        } else if self.matches(&[TokenType::While]) {
            self.while_statement()
        } else if self.matches(&[TokenType::LeftBrace]) {
            let statements = self.block()?;
            Ok(Stmt::Block { statements })
        } else {
            self.expression_statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, Vec<LoxError>> {
        let name = self
            .consume(&TokenType::Identifier, "Expect variable name.")
            .map_err(|error| vec![error])?
            .clone();

        let initializer = if self.matches(&[TokenType::Equal]) {
            self.expression().map_err(|error| vec![error])?
        } else {
            Expr::Literal { value: nil() }
        };

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )
        .map_err(|error| vec![error])?;

        Ok(Stmt::Var {
            name,
            initializer: Box::new(initializer),
        })
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, Vec<LoxError>> {
        let name = self
            .consume(&TokenType::Identifier, &format!("Expect {kind}, name."))
            .map_err(|err| vec![err])?
            .clone();

        self.consume(
            &TokenType::LeftParen,
            &format!("Expect '(' after {kind} name."),
        )
        .map_err(|error| vec![error])?;

        let mut params = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(vec![parser_error(
                        self.peek(),
                        "Can't have more than 255 arguments.",
                    )]);
                }

                let param = self
                    .consume(&TokenType::Identifier, "Expect parameter name.")
                    .map_err(|error| vec![error])?
                    .clone();

                params.push(param);

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(&TokenType::RightParen, "Expect ')' after parameters.")
            .map_err(|error| vec![error])?;

        self.consume(
            &TokenType::LeftBrace,
            &format!("Expect '{{' before {kind} body."),
        )
        .map_err(|error| vec![error])?;
        let body = self.block()?;

        Ok(Stmt::Function { name, params, body })
    }

    fn class_declaration(&mut self) -> Result<Stmt, Vec<LoxError>> {
        let name = self
            .consume(&TokenType::Identifier, "Expect class name.")
            .map_err(|err| vec![err])?
            .clone();

        let superclass = if self.matches(&[TokenType::Less]) {
            let name = self
                .consume(&TokenType::Identifier, "Expect superclass name.")
                .map_err(|err| vec![err])?
                .clone();
            Some(Box::new(Expr::Variable { name }))
        } else {
            None
        };

        self.consume(&TokenType::LeftBrace, "Expect '{' before class body.")
            .map_err(|error| vec![error])?;

        let mut methods = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after class body.")
            .map_err(|error| vec![error])?;

        Ok(Stmt::Class {
            name,
            methods,
            superclass,
        })
    }

    fn declaration(&mut self) -> Result<Stmt, Vec<LoxError>> {
        let res = if self.matches(&[TokenType::Class]) {
            self.class_declaration()
        } else if self.matches(&[TokenType::Fun]) {
            self.function("function")
        } else if self.matches(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if res.is_err() {
            self.synchronize();
        }

        res
    }
}
