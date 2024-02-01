use crate::errors::{error, LoxError};

use super::token::*;

pub struct ScanResult {
    tokens: Vec<Token>,
    errors: Vec<LoxError>,
}

impl ScanResult {
    pub fn unwrap(mut self, errors: &mut Vec<LoxError>) -> Vec<Token> {
        errors.append(&mut self.errors);
        self.tokens
    }
}

pub fn scan_tokens(source: String) -> ScanResult {
    let mut state = State::new(source);
    let mut errors = Vec::new();

    while !state.is_at_end() {
        state.start = state.current;
        if let Err(error) = state.scan_token() {
            errors.push(error);
        }
    }

    state.eof();

    ScanResult {
        tokens: state.tokens,
        errors,
    }
}

struct State {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl State {
    fn new(source: String) -> State {
        State {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => {
                let token = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(token);
            }
            '=' => {
                let token = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token);
            }
            '<' => {
                let token = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                self.add_token(token);
            }
            '>' => {
                let token = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(token);
            }

            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            '"' => {
                return self.string();
            }

            other => {
                if other.is_ascii_digit() {
                    self.number();
                } else if other.is_alpha() {
                    self.identifier();
                } else {
                    return Err(error(self.line, "Unexpected character."));
                }
            }
        }

        Ok(())
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(error(self.line, "Unterminated string."));
        }

        self.advance();

        let value = String::from(&self.source[self.start + 1..self.current - 1]);
        self.add_token(TokenType::String(value));
        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let string = &self.source[self.start..self.current];
        let value = string.parse().unwrap();
        self.add_token(TokenType::Number(value));
    }

    fn identifier(&mut self) {
        while self.peek().is_alpha_or_digit() {
            self.advance();
        }

        let string = &self.source[self.start..self.current];
        let token = string.keyword().unwrap_or(TokenType::Identifier);
        self.add_token(token);
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = String::from(&self.source[self.start..self.current]);
        self.tokens
            .push(Token::new(token_type, lexeme, self.line, self.current));
    }

    fn advance(&mut self) -> char {
        let c = self.get_current_char();
        self.current += c.len_utf8();
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current = self.get_current_char();
        if current != expected {
            return false;
        }

        self.current += current.len_utf8();
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.get_current_char()
    }

    fn peek_next(&self) -> char {
        let current_size = self.get_current_char().len_utf8();
        if self.current + current_size >= self.source.len() {
            return '\0';
        }
        self.get_char(self.current + current_size)
    }

    fn get_current_char(&self) -> char {
        self.get_char(self.current)
    }

    fn get_char(&self, index: usize) -> char {
        self.source[index..].chars().next().unwrap()
    }

    pub fn eof(&mut self) {
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".into(),
            self.line,
            self.current,
        ));
    }
}

trait IsAlpha {
    fn is_alpha(&self) -> bool;
    fn is_alpha_or_digit(&self) -> bool;
}

impl IsAlpha for char {
    fn is_alpha(&self) -> bool {
        self.is_ascii_alphabetic() || *self == '_'
    }

    fn is_alpha_or_digit(&self) -> bool {
        self.is_alpha() || self.is_ascii_digit()
    }
}
