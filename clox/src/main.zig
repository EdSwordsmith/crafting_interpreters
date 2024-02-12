const std = @import("std");

const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var chunk = Chunk.init(allocator);
    defer chunk.deinit();

    // OP_CONSTANT 1.2
    var constant = try chunk.addConstant(1.2);
    try chunk.writeOp(OpCode.Constant, 123);
    try chunk.write(constant, 123);

    // OP_RETURN
    try chunk.writeOp(OpCode.Return, 123);

    chunk.disassemble("test chunk");
}
