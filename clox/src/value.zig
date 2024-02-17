const std = @import("std");

pub const Value = union(enum) {
    number: f64,
    boolean: bool,
    nil,

    pub fn print(value: Value) void {
        switch (value) {
            .number => std.debug.print("{d}", .{value.number}),
            .boolean => std.debug.print("{}", .{value.boolean}),
            .nil => std.debug.print("nil", .{}),
        }
    }

    pub fn number(value: f64) Value {
        return Value{ .number = value };
    }

    pub fn boolean(value: bool) Value {
        return Value{ .boolean = value };
    }

    pub fn nil() Value {
        return Value{ .nil = {} };
    }

    pub fn isFalsey(value: Value) bool {
        return switch (value) {
            .nil => true,
            .boolean => !value.boolean,
            else => false,
        };
    }

    pub fn equal(a: Value, b: Value) bool {
        return std.meta.eql(a, b);
    }
};
