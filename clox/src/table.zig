const std = @import("std");

const Obj = @import("object.zig").Obj;
const LoxValue = @import("value.zig").Value;

pub const TableConfig = struct {
    cmp_strings: bool = false,
};

pub fn Table(comptime Value: type, comptime config: TableConfig) type {
    const Context = struct {
        pub fn hash(_: @This(), key: LoxValue) u64 {
            return switch (key) {
                .boolean => if (key.boolean) 5 else 2,
                .nil => 13,
                .number => @bitCast(key.number),
                .obj => key.obj.data.string.hash,
            };
        }

        pub fn eql(_: @This(), k1: LoxValue, k2: LoxValue) bool {
            return if (config.cmp_strings) std.mem.eql(u8, k1.obj.data.string.chars, k2.obj.data.string.chars) else std.meta.eql(k1, k2);
        }
    };

    return std.HashMap(LoxValue, Value, Context, 75);
}
