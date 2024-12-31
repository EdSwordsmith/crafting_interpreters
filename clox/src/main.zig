const std = @import("std");

const VM = @import("vm.zig").VM;
const Value = @import("value.zig").Value;

pub fn main() !u8 {
    var stack_buffer: [@sizeOf(Value) * 256]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&stack_buffer);
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    // defer std.debug.assert(gpa.deinit() == .ok);

    var vm = try VM.init(gpa.allocator(), fba.allocator());
    defer vm.deinit();

    const file = parseArgs() catch {
        std.debug.print("Usage: clox [path]\n", .{});
        return 64;
    };

    if (file) |file_name| {
        runFile(&vm, file_name) catch |err| switch (err) {
            error.CompileError => return 65,
            error.RuntimeError => return 70,
            error.CouldNotOpen => {
                std.debug.print("Could not open file \"{s}\"", .{file_name});
                return 74;
            },
            error.OutOfMemory => {
                std.debug.print("Not enough memory to read \"{s}\"", .{file_name});
                return 74;
            },
            else => return err,
        };
    } else {
        try repl(&vm);
    }

    return 0;
}

fn parseArgs() error{ShowUsage}!?[]const u8 {
    var file: ?[]const u8 = null;
    var args = std.process.args();
    var argc: usize = 0;

    while (args.next()) |arg| {
        if (argc == 1) file = arg;
        argc += 1;
    }

    if (argc > 2) return error.ShowUsage;

    return file;
}

fn repl(vm: *VM) !void {
    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();

    while (true) {
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        const allocator = arena.allocator();
        defer arena.deinit();

        try stdout.print("> ", .{});

        var line = std.ArrayList(u8).init(allocator);
        defer line.deinit();

        stdin.streamUntilDelimiter(line.writer(), '\n', null) catch |err| switch (err) {
            error.EndOfStream => {
                try stdout.print("\n", .{});
                break;
            },
            else => return err,
        };

        vm.interpret(line.items, allocator) catch {};
    }
}

fn runFile(vm: *VM, file_name: []const u8) !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    const allocator = arena.allocator();
    defer arena.deinit();

    const file = std.fs.cwd().openFile(file_name, .{}) catch {
        return error.CouldNotOpen;
    };
    defer file.close();

    const stat = try file.stat();
    const buffer = try allocator.alloc(u8, stat.size);
    _ = try file.readAll(buffer);

    try vm.interpret(buffer, allocator);
}
