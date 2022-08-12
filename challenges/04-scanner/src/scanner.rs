use crate::Lox;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

pub struct Scanner<'a> {
    lox: &'a mut Lox,
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(lox: &mut Lox, source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            lox,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".into(), self.line));

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
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
                } else if self.matches('*') {
                    let mut open_comments = 1;
                    while open_comments > 0 && !self.is_at_end() {
                        let c = self.advance();
                        match c {
                            '/' => {
                                if self.matches('*') {
                                    open_comments += 1;
                                }
                            }
                            '*' => {
                                if self.matches('/') {
                                    open_comments -= 1;
                                }
                            }
                            '\n' => self.line += 1,
                            _ => {}
                        }
                    }

                    if open_comments > 0 {
                        self.lox.error(self.line, "Unterminated block comment.");
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            '"' => self.string(),

            other => {
                if other.is_ascii_digit() {
                    self.number();
                } else if other.is_alpha() {
                    self.identifier();
                } else {
                    self.lox.error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.lox.error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let value = String::from(&self.source[self.start + 1..self.current - 1]);
        self.add_token(TokenType::String(value));
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
        self.tokens.push(Token::new(token_type, lexeme, self.line));
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
        (&self.source[index..]).chars().next().unwrap()
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

trait Keyword {
    fn keyword(&self) -> Option<TokenType>;
}

impl Keyword for str {
    fn keyword(&self) -> Option<TokenType> {
        match self {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),

            _ => None,
        }
    }
}
