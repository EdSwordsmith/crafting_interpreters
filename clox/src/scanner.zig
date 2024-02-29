const std = @import("std");

pub const TokenType = enum {
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
    Colon,

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
    String,
    Number,

    // Keywords.
    And,
    Case,
    Class,
    Default,
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
    Switch,
    This,
    True,
    Var,
    While,

    Error,
    EOF,
};

pub const Token = struct {
    token_type: TokenType,
    lexeme: []const u8,
    line: usize,
};

pub const Scanner = struct {
    source: []const u8,
    start: usize,
    current: usize,
    line: usize,

    pub fn init(source: []const u8) Scanner {
        return Scanner{
            .source = source,
            .start = 0,
            .current = 0,
            .line = 1,
        };
    }

    pub fn scanToken(self: *Scanner) Token {
        self.skipWhitespace();
        self.start = self.current;

        if (self.isAtEnd())
            return self.makeToken(TokenType.EOF);

        const c: u8 = self.advance();

        if (isDigit(c)) return self.number();
        if (isAlpha(c)) return self.identifier();

        return switch (c) {
            '(' => self.makeToken(TokenType.LeftParen),
            ')' => self.makeToken(TokenType.RightParen),
            '{' => self.makeToken(TokenType.LeftBrace),
            '}' => self.makeToken(TokenType.RightBrace),
            ';' => self.makeToken(TokenType.Semicolon),
            ',' => self.makeToken(TokenType.Comma),
            '.' => self.makeToken(TokenType.Dot),
            '-' => self.makeToken(TokenType.Minus),
            '+' => self.makeToken(TokenType.Plus),
            '/' => self.makeToken(TokenType.Slash),
            '*' => self.makeToken(TokenType.Star),
            ':' => self.makeToken(TokenType.Colon),

            '!' => self.makeToken(if (self.match('=')) TokenType.BangEqual else TokenType.Bang),
            '=' => self.makeToken(if (self.match('=')) TokenType.EqualEqual else TokenType.Equal),
            '<' => self.makeToken(if (self.match('=')) TokenType.LessEqual else TokenType.Less),
            '>' => self.makeToken(if (self.match('=')) TokenType.GreaterEqual else TokenType.Greater),

            '"' => self.string(),

            else => self.errorToken("Unexpected character."),
        };
    }

    fn skipWhitespace(self: *Scanner) void {
        while (true) {
            switch (self.peek()) {
                ' ', '\r', '\t' => {
                    _ = self.advance();
                },

                '\n' => {
                    self.line += 1;
                    _ = self.advance();
                },

                '/' => {
                    if (self.peekNext() == '/') {
                        // A comment goes until the end of the line.
                        while (self.peek() != '\n' and !self.isAtEnd()) _ = self.advance();
                    } else {
                        return;
                    }
                },

                else => return,
            }
        }
    }

    fn peek(self: *const Scanner) u8 {
        return if (self.isAtEnd()) 0 else self.source[self.current];
    }

    fn peekNext(self: *const Scanner) u8 {
        return if (self.isAtEnd()) 0 else self.source[self.current + 1];
    }

    fn advance(self: *Scanner) u8 {
        self.current += 1;
        return self.source[self.current - 1];
    }

    fn match(self: *Scanner, expected: u8) bool {
        if (self.isAtEnd()) return false;
        if (self.source[self.current] != expected) return false;
        self.current += 1;
        return true;
    }

    fn isAtEnd(self: *const Scanner) bool {
        return self.current == self.source.len;
    }

    fn makeToken(self: *const Scanner, token_type: TokenType) Token {
        return Token{
            .line = self.line,
            .token_type = token_type,
            .lexeme = self.source[self.start..self.current],
        };
    }

    fn errorToken(self: *const Scanner, message: []const u8) Token {
        return Token{
            .line = self.line,
            .token_type = TokenType.Error,
            .lexeme = message,
        };
    }

    fn string(self: *Scanner) Token {
        while (self.peek() != '"' and !self.isAtEnd()) {
            if (self.peek() == '\n') self.line += 1;
            _ = self.advance();
        }

        if (self.isAtEnd()) return self.errorToken("Unterminated string.");

        // The closing quote.
        _ = self.advance();
        return self.makeToken(TokenType.String);
    }

    fn number(self: *Scanner) Token {
        while (isDigit(self.peek())) _ = self.advance();

        // Look for a fractional part.
        if (self.peek() == '.' and isDigit(self.peekNext())) {
            // Consume the ".".
            _ = self.advance();

            while (isDigit(self.peek())) _ = self.advance();
        }

        return self.makeToken(TokenType.Number);
    }

    fn identifier(self: *Scanner) Token {
        while (isAlpha(self.peek()) or isDigit(self.peek())) _ = self.advance();
        return self.makeToken(self.identifierType());
    }

    fn identifierType(self: *const Scanner) TokenType {
        return switch (self.source[self.start]) {
            'a' => self.checkKeyword(1, "nd", TokenType.And),

            'c' => if (self.current - self.start > 1) switch (self.source[self.start + 1]) {
                'a' => self.checkKeyword(2, "se", TokenType.Case),
                'l' => self.checkKeyword(2, "ass", TokenType.Class),
                else => TokenType.Identifier,
            } else TokenType.Identifier,

            'd' => self.checkKeyword(1, "efault", TokenType.Default),

            'e' => self.checkKeyword(1, "lse", TokenType.Else),

            'f' => if (self.current - self.start > 1) switch (self.source[self.start + 1]) {
                'a' => self.checkKeyword(2, "lse", TokenType.False),
                'o' => self.checkKeyword(2, "r", TokenType.For),
                'u' => self.checkKeyword(2, "n", TokenType.Fun),
                else => TokenType.Identifier,
            } else TokenType.Identifier,

            'i' => self.checkKeyword(1, "f", TokenType.If),
            'n' => self.checkKeyword(1, "il", TokenType.Nil),
            'o' => self.checkKeyword(1, "r", TokenType.Or),
            'p' => self.checkKeyword(1, "rint", TokenType.Print),
            'r' => self.checkKeyword(1, "eturn", TokenType.Return),

            's' => if (self.current - self.start > 1) switch (self.source[self.start + 1]) {
                'u' => self.checkKeyword(2, "per", TokenType.Super),
                'w' => self.checkKeyword(2, "itch", TokenType.Switch),
                else => TokenType.Identifier,
            } else TokenType.Identifier,

            't' => if (self.current - self.start > 1) switch (self.source[self.start + 1]) {
                'h' => self.checkKeyword(2, "is", TokenType.This),
                'r' => self.checkKeyword(2, "ue", TokenType.True),
                else => TokenType.Identifier,
            } else TokenType.Identifier,

            'v' => self.checkKeyword(1, "ar", TokenType.Var),
            'w' => self.checkKeyword(1, "hile", TokenType.While),
            else => TokenType.Identifier,
        };
    }

    fn checkKeyword(self: *const Scanner, start: usize, rest: []const u8, token_type: TokenType) TokenType {
        if (std.mem.eql(u8, rest, self.source[self.start + start .. self.current])) {
            return token_type;
        }
        return TokenType.Identifier;
    }
};

fn isDigit(c: u8) bool {
    return c >= '0' and c <= '9';
}

fn isAlpha(c: u8) bool {
    return (c >= 'a' and c <= 'z') or
        (c >= 'A' and c <= 'Z') or
        c == '_';
}
