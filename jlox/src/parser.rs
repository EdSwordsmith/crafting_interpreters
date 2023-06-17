use std::matches;

use crate::{
    ast::{Expr, Object},
    errors::{error_with_location, Errors, LoxError},
    scanner::{Token, TokenType},
};

pub fn parse(tokens: Vec<Token>) -> Result<Expr, Errors> {
    let mut state = State::new(tokens);
    match state.expression() {
        Ok(expr) => Ok(expr),
        Err(error) => Err(Errors(vec![error])),
    }
}

fn parser_error(token: &Token, message: &str) -> LoxError {
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
                value: Object::Bool(false),
            })
        } else if self.matches(&[TokenType::True]) {
            Ok(Expr::Literal {
                value: Object::Bool(true),
            })
        } else if self.matches(&[TokenType::Nil]) {
            Ok(Expr::Literal { value: Object::Nil })
        } else if self.matches(&[TokenType::Number(0.0), TokenType::String("".into())]) {
            let obj = match &self.previous().token_type {
                TokenType::Number(value) => Object::Number(*value),
                TokenType::String(value) => Object::String(value.into()),
                _ => unreachable!(),
            };
            Ok(Expr::Literal { value: obj })
        } else if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else {
            Err(parser_error(self.peek(), "Expect expression."))
        }
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            Ok(Expr::Unary {
                operator: self.previous().clone(),
                right: Box::new(self.unary()?),
            })
        } else {
            self.primary()
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

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }
}
