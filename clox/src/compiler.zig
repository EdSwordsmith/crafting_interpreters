const std = @import("std");

const Scanner = @import("scanner.zig").Scanner;
const TokenType = @import("scanner.zig").TokenType;
const Token = @import("scanner.zig").Token;
const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;
const Value = @import("value.zig").Value;
const flags = @import("flags");

const Precedence = enum {
    None,
    Assignment, // =
    Or, // or
    And, // and
    Equality, // == !=
    Comparison, // < > <= >=
    Term, // + -
    Factor, // * /
    Unary, // ! -
    Call, // . ()
    Primary,
};

const ParseFn = *const fn (*Compiler) anyerror!void;

const ParseRule = struct {
    prefix: ?ParseFn = null,
    infix: ?ParseFn = null,
    precedence: Precedence = Precedence.None,
};

fn getRules() std.EnumArray(TokenType, ParseRule) {
    var array = std.EnumArray(TokenType, ParseRule).initFill(ParseRule{});

    array.set(TokenType.LeftParen, .{ .prefix = Compiler.grouping });

    // Unary
    array.set(TokenType.Minus, .{ .prefix = Compiler.unary, .infix = Compiler.binary, .precedence = Precedence.Term });
    array.set(TokenType.Bang, .{ .prefix = Compiler.unary });

    // Binary Operations
    array.set(TokenType.Plus, .{ .infix = Compiler.binary, .precedence = Precedence.Term });
    array.set(TokenType.Slash, .{ .infix = Compiler.binary, .precedence = Precedence.Factor });
    array.set(TokenType.Star, .{ .infix = Compiler.binary, .precedence = Precedence.Factor });

    array.set(TokenType.EqualEqual, .{ .infix = Compiler.binary, .precedence = Precedence.Equality });
    array.set(TokenType.BangEqual, .{ .infix = Compiler.binary, .precedence = Precedence.Equality });
    array.set(TokenType.Greater, .{ .infix = Compiler.binary, .precedence = Precedence.Comparison });
    array.set(TokenType.GreaterEqual, .{ .infix = Compiler.binary, .precedence = Precedence.Comparison });
    array.set(TokenType.Less, .{ .infix = Compiler.binary, .precedence = Precedence.Comparison });
    array.set(TokenType.LessEqual, .{ .infix = Compiler.binary, .precedence = Precedence.Comparison });

    // Literals
    array.set(TokenType.Number, .{ .prefix = Compiler.number });
    array.set(TokenType.True, .{ .prefix = Compiler.literal });
    array.set(TokenType.False, .{ .prefix = Compiler.literal });
    array.set(TokenType.Nil, .{ .prefix = Compiler.literal });

    return array;
}

const rules = getRules();

const Parser = struct {
    current: Token,
    previous: Token,
    hadError: bool,
    panicMode: bool,

    fn init() Parser {
        return Parser{ .current = undefined, .previous = undefined, .hadError = false, .panicMode = false };
    }
};

pub const Compiler = struct {
    chunk: Chunk,
    scanner: Scanner,
    parser: Parser,

    pub fn init(allocator: std.mem.Allocator, source: []const u8) Compiler {
        return Compiler{ .chunk = Chunk.init(allocator), .scanner = Scanner.init(source), .parser = Parser.init() };
    }

    pub fn deinit(self: *Compiler) void {
        self.chunk.deinit();
    }

    pub fn compile(self: *Compiler) !Chunk {
        self.advance();
        try self.expression();
        self.consume(TokenType.EOF, "Expect end of expression.");
        try self.emitOp(OpCode.Return);

        if (self.parser.hadError)
            return error.CompileError;

        if (flags.debug_print_code)
            self.chunk.disassemble("code");

        return self.chunk;
    }

    fn parsePrecedence(self: *Compiler, precedence: Precedence) !void {
        self.advance();

        if (rules.get(self.parser.previous.token_type).prefix) |prefixRule| {
            try prefixRule(self);
        } else {
            self.errorAtPrevious("Expect expression.");
        }

        while (@intFromEnum(precedence) <= @intFromEnum(rules.get(self.parser.current.token_type).precedence)) {
            self.advance();
            if (rules.get(self.parser.previous.token_type).infix) |infixRule| {
                try infixRule(self);
            }
        }
    }

    fn expression(self: *Compiler) !void {
        try self.parsePrecedence(Precedence.Assignment);
    }

    fn number(self: *Compiler) !void {
        const value = try std.fmt.parseFloat(f64, self.parser.previous.lexeme);
        const constant = try self.makeConstant(Value{ .number = value });

        try self.emitOp(OpCode.Constant);
        try self.emitByte(constant);
    }

    fn literal(self: *Compiler) !void {
        switch (self.parser.previous.token_type) {
            .False => try self.emitOp(OpCode.False),
            .Nil => try self.emitOp(OpCode.Nil),
            .True => try self.emitOp(OpCode.True),
            else => return,
        }
    }

    fn grouping(self: *Compiler) !void {
        try self.expression();
        self.consume(TokenType.RightParen, "Expect ')' after expression.");
    }

    fn unary(self: *Compiler) !void {
        const operatorType = self.parser.previous.token_type;

        // Compile the operand.
        try self.parsePrecedence(Precedence.Unary);

        // Emit the operator instruction.
        switch (operatorType) {
            .Minus => try self.emitOp(OpCode.Negate),
            .Bang => try self.emitOp(OpCode.Not),
            else => {},
        }
    }

    fn binary(self: *Compiler) !void {
        const operatorType = self.parser.previous.token_type;
        const rule = rules.getPtrConst(operatorType);
        try self.parsePrecedence(@enumFromInt(@intFromEnum(rule.precedence) + 1));

        switch (operatorType) {
            .Plus => try self.emitOp(OpCode.Add),
            .Minus => try self.emitOp(OpCode.Subtract),
            .Star => try self.emitOp(OpCode.Multiply),
            .Slash => try self.emitOp(OpCode.Divide),

            .EqualEqual => try self.emitOp(OpCode.Equal),
            .BangEqual => {
                try self.emitOp(OpCode.Equal);
                try self.emitOp(OpCode.Not);
            },
            .Greater => try self.emitOp(OpCode.Greater),
            .GreaterEqual => {
                try self.emitOp(OpCode.Less);
                try self.emitOp(OpCode.Not);
            },
            .Less => try self.emitOp(OpCode.Less),
            .LessEqual => {
                try self.emitOp(OpCode.Greater);
                try self.emitOp(OpCode.Not);
            },

            else => {},
        }
    }

    fn emitByte(self: *Compiler, byte: u8) !void {
        try self.chunk.write(byte, self.parser.previous.line);
    }

    fn emitOp(self: *Compiler, op: OpCode) !void {
        try self.chunk.writeOp(op, self.parser.previous.line);
    }

    fn makeConstant(self: *Compiler, value: Value) !u8 {
        const constant = try self.chunk.addConstant(value);
        if (constant > std.math.maxInt(u8)) {
            self.errorAtPrevious("Too many constants in one chunk.");
            return 0;
        }

        return constant;
    }

    fn advance(self: *Compiler) void {
        self.parser.previous = self.parser.current;
        while (true) {
            self.parser.current = self.scanner.scanToken();
            if (self.parser.current.token_type != TokenType.Error) break;

            self.errorAtCurrent(self.parser.current.lexeme);
        }
    }

    fn consume(self: *Compiler, token_type: TokenType, message: []const u8) void {
        if (self.parser.current.token_type == token_type)
            self.advance()
        else
            self.errorAtCurrent(message);
    }

    fn errorAtPrevious(self: *Compiler, message: []const u8) void {
        self.errorAt(&self.parser.previous, message);
    }

    fn errorAtCurrent(self: *Compiler, message: []const u8) void {
        self.errorAt(&self.parser.current, message);
    }

    fn errorAt(self: *Compiler, token: *Token, message: []const u8) void {
        if (self.parser.panicMode) return;
        self.parser.panicMode = true;
        std.debug.print("[line {}] Error", .{token.line});

        if (token.token_type == TokenType.EOF) {
            std.debug.print(" at end", .{});
        } else if (token.token_type != TokenType.Error) {
            std.debug.print(" at {s}", .{token.lexeme});
        }

        std.debug.print(": {s}\n", .{message});
        self.parser.hadError = true;
    }
};
