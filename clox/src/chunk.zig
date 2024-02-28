const std = @import("std");

const Value = @import("./value.zig").Value;

pub const OpCode = enum(u8) {
    Constant,
    Nil,
    True,
    False,
    Pop,
    GetLocal,
    SetLocal,
    GetGlobal,
    DefineGlobal,
    SetGlobal,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Print,
    Jump,
    JumpIfFalse,
    Loop,
    Return,
};

pub const Chunk = struct {
    code: std.ArrayList(u8),
    lines: std.ArrayList(usize),
    constants: std.ArrayList(Value),

    pub fn init(allocator: std.mem.Allocator) Chunk {
        return Chunk{ .code = std.ArrayList(u8).init(allocator), .lines = std.ArrayList(usize).init(allocator), .constants = std.ArrayList(Value).init(allocator) };
    }

    pub fn deinit(self: *Chunk) void {
        self.code.deinit();
        self.lines.deinit();
        self.constants.deinit();
    }

    pub fn writeOp(self: *Chunk, op_code: OpCode, line: usize) !void {
        try self.write(@intFromEnum(op_code), line);
    }

    pub fn write(self: *Chunk, byte: u8, line: usize) !void {
        try self.code.append(byte);
        try self.lines.append(line);
    }

    pub fn addConstant(self: *Chunk, value: Value) !usize {
        try self.constants.append(value);
        return self.constants.items.len - 1;
    }

    pub fn disassemble(self: *const Chunk, name: []const u8) void {
        std.debug.print("== {s} ==\n", .{name});
        var offset: usize = 0;
        while (offset < self.code.items.len) {
            offset = self.disassembleInstruction(offset);
        }
    }

    pub fn disassembleInstruction(self: *const Chunk, offset: usize) usize {
        std.debug.print("{:0>4} ", .{offset});
        if (offset > 0 and self.lines.items[offset] == self.lines.items[offset - 1]) {
            std.debug.print("   | ", .{});
        } else {
            std.debug.print("{: >4} ", .{self.lines.items[offset]});
        }

        if (self.code.items[offset] >= @typeInfo(OpCode).Enum.fields.len) {
            std.debug.print("Unknown opcode {}\n", .{self.code.items[offset]});
            return offset + 1;
        }

        const instruction: OpCode = @enumFromInt(self.code.items[offset]);
        return switch (instruction) {
            .Constant => self.constantInstruction("OP_CONSTANT", offset),
            .Nil => self.simpleInstruction("OP_NIL", offset),
            .True => self.simpleInstruction("OP_TRUE", offset),
            .False => self.simpleInstruction("OP_FALSE", offset),
            .Pop => self.simpleInstruction("OP_POP", offset),

            .GetLocal => self.byteInstruction("OP_GET_LOCAL", offset),
            .SetLocal => self.byteInstruction("OP_SET_LOCAL", offset),

            .GetGlobal => self.constantInstruction("OP_GET_GLOBAL", offset),
            .DefineGlobal => self.constantInstruction("OP_DEFINE_GLOBAL", offset),
            .SetGlobal => self.constantInstruction("OP_SET_GLOBAL", offset),

            .Equal => self.simpleInstruction("OP_EQUAL", offset),
            .Greater => self.simpleInstruction("OP_GREATER", offset),
            .Less => self.simpleInstruction("OP_LESS", offset),

            .Add => self.simpleInstruction("OP_ADD", offset),
            .Subtract => self.simpleInstruction("OP_SUBTRACT", offset),
            .Multiply => self.simpleInstruction("OP_MULTIPLY", offset),
            .Divide => self.simpleInstruction("OP_DIVIDE", offset),

            .Not => self.simpleInstruction("OP_NOT", offset),
            .Negate => self.simpleInstruction("OP_NEGATE", offset),

            .Print => self.simpleInstruction("OP_PRINT", offset),

            .Jump => self.jumpInstruction("OP_JUMP", 1, offset),
            .JumpIfFalse => self.jumpInstruction("OP_JUMP_IF_FALSE", 1, offset),
            .Loop => self.jumpInstruction("OP_LOOP", -1, offset),

            .Return => self.simpleInstruction("OP_RETURN", offset),
        };
    }

    fn constantInstruction(self: *const Chunk, name: []const u8, offset: usize) usize {
        const constant = self.code.items[offset + 1];
        std.debug.print("{s: <16} {: >4} '", .{ name, constant });
        self.constants.items[constant].print();
        std.debug.print("'\n", .{});
        return offset + 2;
    }

    fn simpleInstruction(self: *const Chunk, name: []const u8, offset: usize) usize {
        _ = self;
        std.debug.print("{s}\n", .{name});
        return offset + 1;
    }

    fn byteInstruction(self: *const Chunk, name: []const u8, offset: usize) usize {
        const slot = self.code.items[offset + 1];
        std.debug.print("{s: <16} {: >4}\n", .{ name, slot });
        return offset + 2;
    }

    fn jumpInstruction(self: *const Chunk, name: []const u8, sign: i8, offset: usize) usize {
        const jump: isize = @intCast(@as(u16, self.code.items[offset + 1]) * 256 + @as(u16, self.code.items[offset + 2]));
        const signed_offset: isize = @intCast(offset);
        std.debug.print("{s: <16} {: >4} -> {}\n", .{ name, offset, signed_offset + 3 + sign * jump });
        return offset + 3;
    }
};
