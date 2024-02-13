const std = @import("std");

const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var chunk = Chunk.init(allocator);
    defer chunk.deinit();

    // Added this for to check if the constant long instruction was being used properly
    for (0..258) |_| {
        // OP_CONSTANT 1.2
        try chunk.writeConstant(1.2, 123);
    }

    // OP_RETURN
    try chunk.writeOp(OpCode.Return, 123);

    chunk.disassemble("test chunk");
}
