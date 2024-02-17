const std = @import("std");

const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;
const Value = @import("value.zig").Value;
const Compiler = @import("compiler.zig").Compiler;
const flags = @import("flags");

pub const VMErrors = error{ CompileError, RuntimeError };

pub const VM = struct {
    chunk: *const Chunk,
    ip: [*]const u8,
    stack: std.ArrayList(Value),

    pub fn init(allocator: std.mem.Allocator) VM {
        return VM{ .chunk = undefined, .ip = undefined, .stack = std.ArrayList(Value).init(allocator) };
    }

    pub fn deinit(self: *VM) void {
        self.stack.deinit();
    }

    pub fn interpret(self: *VM, allocator: std.mem.Allocator, source: []const u8) !void {
        var compiler = Compiler.init(allocator, source);
        defer compiler.deinit();

        var chunk = try compiler.compile();
        defer chunk.deinit();

        self.chunk = &chunk;
        self.ip = self.chunk.code.items.ptr;
        try self.run();
    }

    pub fn run(self: *VM) !void {
        while (true) {
            if (flags.debug_trace_execution) {
                std.debug.print("          ", .{});
                for (self.stack.items) |value| {
                    std.debug.print("[ ", .{});
                    value.print();
                    std.debug.print(" ]", .{});
                }
                std.debug.print("\n", .{});

                const offset = @intFromPtr(self.ip) - @intFromPtr(self.chunk.code.items.ptr);
                _ = self.chunk.disassembleInstruction(offset);
            }

            const instruction: OpCode = @enumFromInt(self.readByte());
            switch (instruction) {
                .Constant => {
                    const constant = self.readConstant();
                    try self.stack.append(constant);
                },

                .Nil => try self.stack.append(Value.nil()),
                .True => try self.stack.append(Value.boolean(true)),
                .False => try self.stack.append(Value.boolean(false)),

                .Equal => {
                    const a = self.stack.pop();
                    const b = self.stack.pop();
                    try self.stack.append(Value.boolean(a.equal(b)));
                },
                .Greater => try self.binaryOp(greater),
                .Less => try self.binaryOp(less),

                .Add => try self.binaryOp(add),
                .Subtract => try self.binaryOp(sub),
                .Multiply => try self.binaryOp(mul),
                .Divide => try self.binaryOp(div),

                .Not => try self.stack.append(Value.boolean(self.stack.pop().isFalsey())),

                .Negate => {
                    switch (self.peek(0)) {
                        .number => {
                            const value = self.stack.pop();
                            try self.stack.append(Value.number(-value.number));
                        },

                        else => {
                            self.runtimeError("Operand must be a number.", .{});
                            return error.RuntimeError;
                        },
                    }
                },

                .Return => {
                    self.stack.pop().print();
                    std.debug.print("\n", .{});
                    return;
                },
            }
        }
    }

    fn readByte(self: *VM) u8 {
        var byte = self.ip[0];
        self.ip += 1;
        return byte;
    }

    fn readConstant(self: *VM) Value {
        return self.chunk.constants.items[self.readByte()];
    }

    fn peek(self: *const VM, distance: usize) Value {
        return self.stack.items[self.stack.items.len - 1 - distance];
    }

    fn runtimeError(self: *VM, comptime format: []const u8, args: anytype) void {
        std.debug.print(format, args);
        const offset = @intFromPtr(self.ip) - @intFromPtr(self.chunk.code.items.ptr) - 1;
        const line = self.chunk.lines.items[offset];
        std.debug.print("\n[line {}] in script\n", .{line});
        self.stack.deinit();
    }

    fn binaryOp(self: *VM, comptime op: fn (f64, f64) Value) !void {
        const numbers = switch (self.peek(0)) {
            .number => switch (self.peek(1)) {
                .number => true,
                else => false,
            },
            else => false,
        };

        if (!numbers) {
            self.runtimeError("Operands must be numbers.", .{});
            return error.RuntimeError;
        }

        const b = self.stack.pop().number;
        const a = self.stack.pop().number;
        try self.stack.append(op(a, b));
    }
};

fn add(a: f64, b: f64) Value {
    return Value.number(a + b);
}

fn sub(a: f64, b: f64) Value {
    return Value.number(a - b);
}

fn mul(a: f64, b: f64) Value {
    return Value.number(a * b);
}

fn div(a: f64, b: f64) Value {
    return Value.number(a / b);
}

fn greater(a: f64, b: f64) Value {
    return Value.boolean(a > b);
}

fn less(a: f64, b: f64) Value {
    return Value.boolean(a < b);
}
