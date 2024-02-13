const std = @import("std");

const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;
const Value = @import("value.zig").Value;
const printValue = @import("value.zig").printValue;

pub const Errors = error{ CompileError, RuntimeError };

const debug_trace_execution = true;

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

    pub fn interpret(self: *VM, chunk: *const Chunk) !void {
        self.chunk = chunk;
        self.ip = chunk.code.items.ptr;
        try self.run();
    }

    pub fn run(self: *VM) !void {
        while (true) {
            if (debug_trace_execution) {
                std.debug.print("          ", .{});
                for (self.stack.items) |value| {
                    std.debug.print("[ ", .{});
                    printValue(value);
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

                .Add => {
                    const b = self.stack.pop();
                    self.stack.items[self.stack.items.len - 1] += b;
                },

                .Subtract => {
                    const b = self.stack.pop();
                    self.stack.items[self.stack.items.len - 1] -= b;
                },

                .Multiply => {
                    const b = self.stack.pop();
                    self.stack.items[self.stack.items.len - 1] *= b;
                },

                .Divide => {
                    const b = self.stack.pop();
                    const a = self.stack.pop();
                    try self.stack.append(a / b);
                },

                .Negate => {
                    self.stack.items[self.stack.items.len - 1] *= -1;
                },

                .Return => {
                    printValue(self.stack.pop());
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
};
