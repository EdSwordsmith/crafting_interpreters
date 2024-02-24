const std = @import("std");

const Obj = @import("object.zig").Obj;

pub const Value = union(enum) {
    number: f64,
    boolean: bool,
    obj: *Obj,
    nil,

    pub fn print(value: Value) void {
        switch (value) {
            .number => std.debug.print("{d}", .{value.number}),
            .boolean => std.debug.print("{}", .{value.boolean}),
            .obj => value.obj.print(),
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

    pub fn obj(value: *Obj) Value {
        return Value{ .obj = value };
    }

    pub fn isString(value: Value) bool {
        return value == .obj and value.obj.data == .string;
    }

    pub fn isFalsey(value: Value) bool {
        return switch (value) {
            .nil => true,
            .boolean => !value.boolean,
            else => false,
        };
    }
};
