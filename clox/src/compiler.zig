const std = @import("std");

const Scanner = @import("scanner.zig").Scanner;
const TokenType = @import("scanner.zig").TokenType;
const Token = @import("scanner.zig").Token;
const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;
const Value = @import("value.zig").Value;
const ObjList = @import("object.zig").ObjList;
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

const ParseFn = *const fn (*Compiler, bool) anyerror!void;

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
    array.set(TokenType.String, .{ .prefix = Compiler.string });

    array.set(TokenType.Identifier, .{ .prefix = Compiler.variable });

    return array;
}

const rules = getRules();

const Parser = struct {
    current: Token = undefined,
    previous: Token = undefined,
    had_error: bool = false,
    panic_mode: bool = false,
};

const Local = struct {
    name: Token,
    depth: isize,
};

pub const Compiler = struct {
    allocator: std.mem.Allocator,
    objects: *ObjList,
    chunk: Chunk,
    scanner: Scanner,
    parser: Parser = .{},

    locals: [256]Local = undefined,
    local_count: usize = 0,
    scope_depth: usize = 0,

    pub fn init(allocator: std.mem.Allocator, objects: *ObjList, source: []const u8) Compiler {
        return Compiler{
            .allocator = allocator,
            .objects = objects,
            .chunk = Chunk.init(allocator),
            .scanner = Scanner.init(source),
        };
    }

    pub fn deinit(self: *Compiler) void {
        self.chunk.deinit();
    }

    pub fn compile(self: *Compiler) !Chunk {
        self.advance();

        while (!self.match(TokenType.EOF)) {
            try self.declaration();
        }

        try self.emitOp(OpCode.Return);

        if (self.parser.had_error)
            return error.CompileError;

        if (flags.debug_print_code)
            self.chunk.disassemble("code");

        return self.chunk;
    }

    fn parsePrecedence(self: *Compiler, precedence: Precedence) !void {
        self.advance();
        const can_assign = @intFromEnum(precedence) <= @intFromEnum(Precedence.Assignment);

        if (rules.get(self.parser.previous.token_type).prefix) |prefixRule| {
            try prefixRule(self, can_assign);
        } else {
            self.errorAtPrevious("Expect expression.");
        }

        while (@intFromEnum(precedence) <= @intFromEnum(rules.get(self.parser.current.token_type).precedence)) {
            self.advance();
            if (rules.get(self.parser.previous.token_type).infix) |infixRule| {
                try infixRule(self, can_assign);
            }
        }

        if (can_assign and self.match(TokenType.Equal)) {
            self.errorAtPrevious("Invalid assignment target.");
        }
    }

    fn expression(self: *Compiler) !void {
        try self.parsePrecedence(Precedence.Assignment);
    }

    fn number(self: *Compiler, _: bool) !void {
        const value = try std.fmt.parseFloat(f64, self.parser.previous.lexeme);
        const constant = try self.makeConstant(Value.number(value));

        try self.emitOp(OpCode.Constant);
        try self.emitByte(constant);
    }

    fn literal(self: *Compiler, _: bool) !void {
        switch (self.parser.previous.token_type) {
            .False => try self.emitOp(OpCode.False),
            .Nil => try self.emitOp(OpCode.Nil),
            .True => try self.emitOp(OpCode.True),
            else => return,
        }
    }

    fn string(self: *Compiler, _: bool) !void {
        const len = self.parser.previous.lexeme.len;
        const chars = try self.objects.allocator.dupe(u8, self.parser.previous.lexeme[1 .. len - 1]);
        const obj = try self.objects.newString(chars);
        const constant = try self.makeConstant(Value.obj(obj));

        try self.emitOp(OpCode.Constant);
        try self.emitByte(constant);
    }

    fn grouping(self: *Compiler, _: bool) !void {
        try self.expression();
        self.consume(TokenType.RightParen, "Expect ')' after expression.");
    }

    fn unary(self: *Compiler, _: bool) !void {
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

    fn binary(self: *Compiler, _: bool) !void {
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

    fn printStatement(self: *Compiler) !void {
        try self.expression();
        self.consume(TokenType.Semicolon, "Expect ';' after value.");
        try self.emitOp(OpCode.Print);
    }

    fn block(self: *Compiler) !void {
        while (!self.check(TokenType.RightBrace) and !self.check(TokenType.EOF)) {
            try self.declaration();
        }

        self.consume(TokenType.RightBrace, "Expect '}' after block.");
    }

    fn beginScope(self: *Compiler) void {
        self.scope_depth += 1;
    }

    fn endScope(self: *Compiler) !void {
        self.scope_depth -= 1;
        while (self.local_count > 0 and self.locals[self.local_count - 1].depth > self.scope_depth) {
            try self.emitOp(OpCode.Pop);
            self.local_count -= 1;
        }
    }

    fn expressionStatement(self: *Compiler) !void {
        try self.expression();
        self.consume(TokenType.Semicolon, "Expect ';' after value.");
        try self.emitOp(OpCode.Pop);
    }

    fn statement(self: *Compiler) !void {
        if (self.match(TokenType.Print)) {
            try self.printStatement();
        } else if (self.match(TokenType.LeftBrace)) {
            self.beginScope();
            try self.block();
            try self.endScope();
        } else {
            try self.expressionStatement();
        }
    }

    fn identifierConstant(self: *Compiler, name: *const Token) !u8 {
        const chars = try self.objects.allocator.dupe(u8, name.lexeme);
        const obj = try self.objects.newString(chars);
        const constant = try self.makeConstant(Value.obj(obj));
        return constant;
    }

    fn parseVariable(self: *Compiler, error_message: []const u8) !u8 {
        self.consume(TokenType.Identifier, error_message);

        self.declareVariable();
        if (self.scope_depth > 0) return 0;

        return try self.identifierConstant(&self.parser.previous);
    }

    fn resolveLocal(self: *Compiler, name: *const Token) ?u8 {
        var i: isize = @intCast(self.local_count);
        i -= 1;

        while (i >= 0) : (i -= 1) {
            const local = &self.locals[@intCast(i)];
            if (std.mem.eql(u8, name.lexeme, local.name.lexeme)) {
                if (local.depth == -1)
                    self.errorAtPrevious("Can't read local variable in its own initializer.");

                return @intCast(i);
            }
        }

        return null;
    }

    fn namedVariable(self: *Compiler, name: Token, can_assign: bool) !void {
        var arg: u8 = 0;
        var set_op = OpCode.SetLocal;
        var get_op = OpCode.GetLocal;

        if (self.resolveLocal(&name)) |local| {
            arg = local;
        } else {
            arg = try self.identifierConstant(&name);
            set_op = OpCode.SetGlobal;
            get_op = OpCode.GetGlobal;
        }

        if (can_assign and self.match(TokenType.Equal)) {
            try self.expression();
            try self.emitOp(set_op);
            try self.emitByte(arg);
        } else {
            try self.emitOp(get_op);
            try self.emitByte(arg);
        }
    }

    fn variable(self: *Compiler, can_assign: bool) !void {
        try self.namedVariable(self.parser.previous, can_assign);
    }

    fn declareVariable(self: *Compiler) void {
        if (self.scope_depth == 0) return;
        const name = &self.parser.previous;

        var i: isize = @intCast(self.local_count);
        i -= 1;

        while (i >= 0) : (i -= 1) {
            const local = &self.locals[@intCast(i)];
            if (local.depth != -1 and local.depth < self.scope_depth)
                break;

            if (std.mem.eql(u8, name.lexeme, local.name.lexeme))
                self.errorAtPrevious("Already a variable with this name in this scope.");
        }

        self.addLocal(name.*);
    }

    fn addLocal(self: *Compiler, name: Token) void {
        if (self.local_count == 256) {
            self.errorAtPrevious("Too many local variables in function.");
            return;
        }

        const local = &self.locals[self.local_count];
        self.local_count += 1;
        local.name = name;
        local.depth = -1;
    }

    fn varDeclaration(self: *Compiler) !void {
        const global = try self.parseVariable("Expect variable name.");

        if (self.match(TokenType.Equal)) {
            try self.expression();
        } else {
            try self.emitOp(OpCode.Nil);
        }

        self.consume(TokenType.Semicolon, "Expect ';' after variable declaration.");

        if (self.scope_depth > 0) {
            self.locals[self.local_count - 1].depth = @intCast(self.scope_depth);
            return;
        }

        try self.emitOp(OpCode.DefineGlobal);
        try self.emitByte(global);
    }

    fn declaration(self: *Compiler) anyerror!void {
        if (self.match(TokenType.Var)) {
            try self.varDeclaration();
        } else {
            try self.statement();
        }

        if (self.parser.panic_mode) self.synchronize();
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

        return @as(u8, @truncate(constant));
    }

    fn synchronize(self: *Compiler) void {
        self.parser.panic_mode = false;

        while (self.parser.current.token_type != TokenType.EOF) {
            if (self.parser.previous.token_type == TokenType.Semicolon) return;
            switch (self.parser.current.token_type) {
                .Class, .Fun, .Var, .For, .If, .While, .Print, .Return => return,
                else => {},
            }

            self.advance();
        }
    }

    fn advance(self: *Compiler) void {
        self.parser.previous = self.parser.current;
        while (true) {
            self.parser.current = self.scanner.scanToken();
            if (self.parser.current.token_type != TokenType.Error) break;

            self.errorAtCurrent(self.parser.current.lexeme);
        }
    }

    fn check(self: *const Compiler, token_type: TokenType) bool {
        return self.parser.current.token_type == token_type;
    }

    fn match(self: *Compiler, token_type: TokenType) bool {
        if (!self.check(token_type)) return false;
        self.advance();
        return true;
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
        if (self.parser.panic_mode) return;
        self.parser.panic_mode = true;
        std.debug.print("[line {}] Error", .{token.line});

        if (token.token_type == TokenType.EOF) {
            std.debug.print(" at end", .{});
        } else if (token.token_type != TokenType.Error) {
            std.debug.print(" at {s}", .{token.lexeme});
        }

        std.debug.print(": {s}\n", .{message});
        self.parser.had_error = true;
    }
};
