const std = @import("std");

const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;
const VM = @import("vm.zig").VM;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var stack_buffer: [256]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&stack_buffer);
    var vm = VM.init(fba.allocator());
    defer vm.deinit();

    var chunk = Chunk.init(allocator);
    defer chunk.deinit();

    // OP_CONSTANT 1.2
    var constant = try chunk.addConstant(1.2);
    try chunk.writeOp(OpCode.Constant, 123);
    try chunk.write(constant, 123);

    // OP_CONSTANT 3.4
    constant = try chunk.addConstant(3.4);
    try chunk.writeOp(OpCode.Constant, 123);
    try chunk.write(constant, 123);

    // OP_ADD
    try chunk.writeOp(OpCode.Add, 123);

    // OP_CONSTANT 5.6
    constant = try chunk.addConstant(5.6);
    try chunk.writeOp(OpCode.Constant, 123);
    try chunk.write(constant, 123);

    // OP_DIVIDE
    try chunk.writeOp(OpCode.Divide, 123);

    // OP_NEGATE
    try chunk.writeOp(OpCode.Negate, 123);

    // OP_RETURN
    try chunk.writeOp(OpCode.Return, 123);

    chunk.disassemble("test chunk");
    try vm.interpret(&chunk);
}
