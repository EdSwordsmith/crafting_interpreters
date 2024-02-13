const std = @import("std");

const Value = @import("./value.zig").Value;
const printValue = @import("./value.zig").printValue;

pub const OpCode = enum(u8) { Constant, Return };

pub const Chunk = struct {
    const LineData = struct { line: usize, offset: usize };

    code: std.ArrayList(u8),
    lines: std.ArrayList(LineData),
    constants: std.ArrayList(Value),

    pub fn init(allocator: std.mem.Allocator) Chunk {
        return Chunk{ .code = std.ArrayList(u8).init(allocator), .lines = std.ArrayList(LineData).init(allocator), .constants = std.ArrayList(Value).init(allocator) };
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

        if (self.lines.items.len == 0 or self.lines.getLast().line != line)
            try self.lines.append(LineData{ .line = line, .offset = self.code.items.len - 1 });
    }

    pub fn getLine(self: *const Chunk, offset: usize) usize {
        const line_count = self.lines.items.len;
        var high: usize = line_count - 1;
        var low: usize = 0;

        while (low < high) {
            const index = low + high / 2;
            const start = self.lines.items[index].offset;

            if (start > offset) {
                high = index - 1;
            } else if (index == line_count - 1 or self.lines.items[index + 1].offset > offset) {
                low = index;
                break;
            } else {
                low = index + 1;
            }
        }

        return self.lines.items[low].line;
    }

    pub fn addConstant(self: *Chunk, value: Value) !u8 {
        try self.constants.append(value);
        return @as(u8, @truncate(self.constants.items.len - 1));
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
        const line = self.getLine(offset);
        if (offset > 0 and line == self.getLine(offset - 1)) {
            std.debug.print("   | ", .{});
        } else {
            std.debug.print("{: >4} ", .{line});
        }

        if (self.code.items[offset] >= @typeInfo(OpCode).Enum.fields.len) {
            std.debug.print("Unknown opcode {}\n", .{self.code.items[offset]});
            return offset + 1;
        }

        const instruction: OpCode = @enumFromInt(self.code.items[offset]);
        switch (instruction) {
            .Constant => {
                return self.constantInstruction("OP_CONSTANT", offset);
            },

            .Return => {
                return self.simpleInstruction("OP_RETURN", offset);
            },
        }
    }

    fn constantInstruction(self: *const Chunk, name: []const u8, offset: usize) usize {
        var constant = self.code.items[offset + 1];
        const padding = 16 - name.len;
        std.debug.print("{s}", .{name});
        for (padding) |_| std.debug.print(" ", .{});
        std.debug.print(" {: >4} '", .{constant});

        printValue(self.constants.items[constant]);
        std.debug.print("'\n", .{});
        return offset + 2;
    }

    fn simpleInstruction(self: *const Chunk, name: []const u8, offset: usize) usize {
        _ = self;
        std.debug.print("{s}\n", .{name});
        return offset + 1;
    }
};
