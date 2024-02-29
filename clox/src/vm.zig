const std = @import("std");

const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;
const Value = @import("value.zig").Value;
const ObjList = @import("object.zig").ObjList;
const Compiler = @import("compiler.zig").Compiler;
const Table = @import("table.zig").Table;
const flags = @import("flags");

const ValueTable = Table(Value, .{});

pub const VM = struct {
    chunk: *const Chunk,
    ip: [*]const u8,
    stack: std.ArrayList(Value),
    objects: ObjList,
    globals: ValueTable,

    pub fn init(object_allocator: std.mem.Allocator, stack_allocator: std.mem.Allocator) !VM {
        var stack = std.ArrayList(Value).init(stack_allocator);
        try stack.ensureTotalCapacityPrecise(256);

        return VM{
            .objects = ObjList.init(object_allocator),
            .chunk = undefined,
            .ip = undefined,
            .stack = stack,
            .globals = ValueTable.init(object_allocator),
        };
    }

    pub fn deinit(self: *VM) void {
        self.stack.deinit();
        self.objects.deinit();
        self.globals.deinit();
    }

    pub fn interpret(self: *VM, source: []const u8, allocator: std.mem.Allocator) !void {
        var compiler = Compiler.init(allocator, &self.objects, source);
        defer compiler.deinit();

        var chunk = try compiler.compile();
        // defer chunk.deinit();

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
                .Pop => {
                    _ = self.stack.pop();
                },

                .Dup => {
                    try self.stack.append(self.stack.items[self.stack.items.len - 1]);
                },
                .GetLocal => {
                    const local = self.readByte();
                    try self.stack.append(self.stack.items[local]);
                },
                .SetLocal => {
                    const local = self.readByte();
                    self.stack.items[local] = self.peek(0);
                },

                .GetGlobal => {
                    const name = self.readConstant().obj;
                    const value = self.globals.get(name);
                    if (value) |v| {
                        try self.stack.append(v);
                    } else {
                        self.runtimeError("Undefined variable '{s}'.", .{name.data.string.chars});
                        return error.RuntimeError;
                    }
                },
                .DefineGlobal => {
                    const name = self.readConstant().obj;
                    try self.globals.put(name, self.stack.pop());
                },
                .SetGlobal => {
                    const name = self.readConstant().obj;
                    if (self.globals.getPtr(name)) |value| {
                        value.* = self.peek(0);
                    } else {
                        self.runtimeError("Undefined variable '{s}'.", .{name.data.string.chars});
                        return error.RuntimeError;
                    }
                },

                .Equal => {
                    const a = self.stack.pop();
                    const b = self.stack.pop();
                    try self.stack.append(Value.boolean(std.meta.eql(a, b)));
                },
                .Greater => {
                    if (self.peek(0) != .number or self.peek(1) != .number) {
                        self.runtimeError("Operands must be numbers.", .{});
                        return error.RuntimeError;
                    }

                    const b = self.stack.pop().number;
                    const a = self.stack.pop().number;
                    try self.stack.append(Value.boolean(a > b));
                },
                .Less => {
                    if (self.peek(0) != .number or self.peek(1) != .number) {
                        self.runtimeError("Operands must be numbers.", .{});
                        return error.RuntimeError;
                    }

                    const b = self.stack.pop().number;
                    const a = self.stack.pop().number;
                    try self.stack.append(Value.boolean(a < b));
                },

                .Add => {
                    if (self.peek(0) == .number and self.peek(1) == .number) {
                        const b = self.stack.pop().number;
                        const a = self.stack.pop().number;
                        try self.stack.append(Value.number(a + b));
                    } else if (self.peek(0).isString() and self.peek(1).isString()) {
                        const b: []const u8 = self.stack.pop().obj.data.string.chars;
                        const a: []const u8 = self.stack.pop().obj.data.string.chars;

                        var result = try self.objects.allocator.alloc(u8, a.len + b.len);
                        @memcpy(result[0..a.len], a);
                        @memcpy(result[a.len..], b);

                        const obj = try self.objects.newString(result);
                        try self.stack.append(Value.obj(obj));
                    } else {
                        self.runtimeError("Operands must be numbers.", .{});
                        return error.RuntimeError;
                    }
                },
                .Subtract => {
                    if (self.peek(0) != .number or self.peek(1) != .number) {
                        self.runtimeError("Operands must be numbers.", .{});
                        return error.RuntimeError;
                    }

                    const b = self.stack.pop().number;
                    const a = self.stack.pop().number;
                    try self.stack.append(Value.number(a - b));
                },
                .Multiply => {
                    if (self.peek(0) != .number or self.peek(1) != .number) {
                        self.runtimeError("Operands must be numbers.", .{});
                        return error.RuntimeError;
                    }

                    const b = self.stack.pop().number;
                    const a = self.stack.pop().number;
                    try self.stack.append(Value.number(a * b));
                },
                .Divide => {
                    if (self.peek(0) != .number or self.peek(1) != .number) {
                        self.runtimeError("Operands must be numbers.", .{});
                        return error.RuntimeError;
                    }

                    const b = self.stack.pop().number;
                    const a = self.stack.pop().number;
                    try self.stack.append(Value.number(a / b));
                },

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

                .Print => {
                    self.stack.pop().print();
                    std.debug.print("\n", .{});
                },

                .Jump => {
                    const offset = self.readShort();
                    self.ip += offset;
                },

                .JumpIfFalse => {
                    const offset = self.readShort();
                    if (self.peek(0).isFalsey())
                        self.ip += offset;
                },

                .Loop => {
                    const offset = self.readShort();
                    self.ip -= offset;
                },

                .Return => return,
            }
        }
    }

    fn readByte(self: *VM) u8 {
        const byte = self.ip[0];
        self.ip += 1;
        return byte;
    }

    fn readShort(self: *VM) u16 {
        const short: u16 = @as(u16, self.ip[0]) * 256 + @as(u16, self.ip[1]);
        self.ip += 2;
        return short;
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
};
