const std = @import("std");

const Obj = @import("object.zig").Obj;

pub const TableConfig = struct {
    cmp_strings: bool = false,
};

pub fn Table(comptime Value: type, comptime config: TableConfig) type {
    const Context = struct {
        pub fn hash(_: @This(), key: *Obj) u64 {
            // Only string objects should be used as key.
            std.debug.assert(key.data == .string);
            return key.data.string.hash;
        }

        pub fn eql(_: @This(), k1: *Obj, k2: *Obj) bool {
            return if (config.cmp_strings) std.mem.eql(u8, k1.data.string.chars, k2.data.string.chars) else std.meta.eql(k1, k2);
        }
    };

    return std.HashMap(*Obj, Value, Context, 75);
}
