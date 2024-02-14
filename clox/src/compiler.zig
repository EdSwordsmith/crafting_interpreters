const std = @import("std");

const Scanner = @import("scanner.zig").Scanner;
const TokenType = @import("scanner.zig").TokenType;

pub fn compile(source: []const u8) void {
    var scanner = Scanner.init(source);
    var line: usize = 0;
    while (true) {
        const token = scanner.scanToken();

        if (token.line != line) {
            std.debug.print("{: >4} ", .{token.line});
            line = token.line;
        } else {
            std.debug.print("   | ", .{});
        }

        std.debug.print("{d:<2} {} {s}\n", .{ @intFromEnum(token.token_type), token.lexeme.len, token.lexeme });

        if (token.token_type == TokenType.EOF) break;
    }
}
