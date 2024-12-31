const std = @import("std");

const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;
const Value = @import("value.zig").Value;
const ObjList = @import("object.zig").ObjList;
const Obj = @import("object.zig").Obj;
const compiler = @import("compiler.zig");
const Table = @import("table.zig").Table;
const flags = @import("flags");

const ValueTable = Table(Value, .{});

const CallFrame = struct {
    function: *Obj,
    ip: [*]const u8,
    slots: [*]Value,
};

fn clock(arg_count: u8, args: [*]Value) Value {
    _ = arg_count;
    _ = args;
    return Value.number(@as(f64, @floatFromInt(std.time.milliTimestamp())) / 1000);
}

pub const VM = struct {
    stack: std.ArrayList(Value),
    objects: ObjList,
    globals: ValueTable,
    // TODO: move the array len to a constant (see max stack size too)
    frames: [64]CallFrame = undefined,
    frame_count: usize = 0,

    pub fn init(object_allocator: std.mem.Allocator, stack_allocator: std.mem.Allocator) !VM {
        var stack = std.ArrayList(Value).init(stack_allocator);
        try stack.ensureTotalCapacityPrecise(256);

        var vm = VM{
            .objects = ObjList.init(object_allocator),
            .stack = stack,
            .globals = ValueTable.init(object_allocator),
        };

        try vm.defineNative("clock", clock);

        return vm;
    }

    pub fn deinit(self: *VM) void {
        self.stack.deinit();
        // self.objects.deinit();
        self.globals.deinit();
    }

    pub fn interpret(self: *VM, source: []const u8, allocator: std.mem.Allocator) !void {
        const function = try compiler.compile(allocator, &self.objects, source);

        try self.stack.append(Value.obj(function));
        _ = self.call(function, 0);

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

                const chunk = &self.frame().function.data.function.chunk;
                const offset = @intFromPtr(self.frame().ip) - @intFromPtr(chunk.code.items.ptr);
                _ = chunk.disassembleInstruction(offset);
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

                .GetLocal => {
                    const slot = self.readByte();
                    try self.stack.append(self.frame().slots[slot]);
                },
                .SetLocal => {
                    const slot = self.readByte();
                    self.frame().slots[slot] = self.peek(0);
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
                    self.frame().ip += offset;
                },

                .JumpIfFalse => {
                    const offset = self.readShort();
                    if (self.peek(0).isFalsey())
                        self.frame().ip += offset;
                },

                .Loop => {
                    const offset = self.readShort();
                    self.frame().ip -= offset;
                },

                .Call => {
                    const arg_count = self.readByte();
                    const ok = try self.callValue(self.peek(arg_count), arg_count);
                    if (!ok)
                        return error.RuntimeError;
                },

                .Return => {
                    const result = self.stack.pop();
                    const len = (@intFromPtr(self.frame().slots) - @intFromPtr(self.stack.items.ptr)) / @sizeOf(Value);
                    self.frame_count -= 1;
                    if (self.frame_count == 0) {
                        _ = self.stack.pop();
                        return;
                    }

                    self.stack.shrinkRetainingCapacity(len);
                    try self.stack.append(result);
                },
            }
        }
    }

    fn frame(self: *VM) *CallFrame {
        return &self.frames[self.frame_count - 1];
    }

    fn readByte(self: *VM) u8 {
        const byte = self.frame().ip[0];
        self.frame().ip += 1;
        return byte;
    }

    fn readShort(self: *VM) u16 {
        const short: u16 = @as(u16, self.frame().ip[0]) * 256 + @as(u16, self.frame().ip[1]);
        self.frame().ip += 2;
        return short;
    }

    fn readConstant(self: *VM) Value {
        return self.frame().function.data.function.chunk.constants.items[self.readByte()];
    }

    fn peek(self: *const VM, distance: usize) Value {
        return self.stack.items[self.stack.items.len - 1 - distance];
    }

    fn call(self: *VM, function: *Obj, arg_count: u8) bool {
        if (arg_count != function.data.function.arity) {
            self.runtimeError("Expected {} arguments but got {}.", .{ function.data.function.arity, arg_count });
            return false;
        }

        if (self.frame_count == 64) {
            self.runtimeError("Stack overflow.", .{});
            return false;
        }

        const call_frame = &self.frames[self.frame_count];
        self.frame_count += 1;
        call_frame.function = function;
        call_frame.ip = function.data.function.chunk.code.items.ptr;
        const offset = self.stack.items.len - arg_count - 1;
        call_frame.slots = self.stack.items.ptr + offset;
        return true;
    }

    fn callValue(self: *VM, callee: Value, arg_count: u8) !bool {
        if (callee == .obj) {
            switch (callee.obj.data) {
                .function => return self.call(callee.obj, arg_count),
                .native => |native| {
                    const offset = self.stack.items.len - arg_count;
                    const args = self.stack.items.ptr + offset;
                    const result = native(arg_count, args);
                    self.stack.shrinkRetainingCapacity(self.stack.items.len - arg_count - 1);
                    try self.stack.append(result);
                    return true;
                },
                else => {},
            }
        }

        self.runtimeError("Can only call functions and classes.", .{});
        return false;
    }

    fn runtimeError(self: *VM, comptime format: []const u8, args: anytype) void {
        std.debug.print(format, args);
        std.debug.print("\n", .{});

        var i = @as(isize, @intCast(self.frame_count)) - 1;
        while (i >= 0) : (i -= 1) {
            const call_frame = self.frames[@as(usize, @intCast(i))];
            const function = call_frame.function;
            const chunk = &function.data.function.chunk;
            const offset = @intFromPtr(call_frame.ip) - @intFromPtr(chunk.code.items.ptr) - 1;
            const line = chunk.lines.items[offset];
            std.debug.print("[line {}] in ", .{line});
            if (function.data.function.name) |name| {
                std.debug.print("{s}()\n", .{name.data.string.chars});
            } else {
                std.debug.print("script\n", .{});
            }
        }

        self.stack.shrinkRetainingCapacity(0);
        self.frame_count = 0;
    }

    fn defineNative(self: *VM, name: []const u8, function: Obj.NativeFn) !void {
        const string = try self.objects.newString(name);
        try self.stack.append(Value.obj(string));
        const native = try self.objects.newNative(function);
        try self.stack.append(Value.obj(native));
        try self.globals.put(self.stack.items[0].obj, self.stack.items[1]);
        _ = self.stack.pop();
        _ = self.stack.pop();
    }
};
