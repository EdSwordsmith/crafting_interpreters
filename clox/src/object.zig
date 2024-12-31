const std = @import("std");

const Table = @import("table.zig").Table;
const Chunk = @import("chunk.zig").Chunk;
const Value = @import("value.zig").Value;

pub const Obj = struct {
    pub const String = struct {
        chars: []const u8,
        hash: u32,

        pub fn init(chars: []const u8) String {
            const hash = std.hash.Fnv1a_32.hash(chars);
            return String{
                .chars = chars,
                .hash = hash,
            };
        }
    };

    pub const Function = struct {
        arity: u8 = 0,
        chunk: Chunk,
        name: ?*Obj = null,

        pub fn init(allocator: std.mem.Allocator) Function {
            return Function{ .chunk = Chunk.init(allocator) };
        }

        pub fn deinit(self: *Function) void {
            self.chunk.deinit();
        }

        pub fn print(self: *const Function) void {
            if (self.name) |name| {
                std.debug.print("<fn {s}>", .{name.data.string.chars});
            } else {
                std.debug.print("<script>", .{});
            }
        }
    };

    pub const NativeFn = *const fn (arg_count: u8, args: [*]Value) Value;

    pub const Data = union(enum) {
        string: String,
        function: Function,
        native: NativeFn,
    };

    next: ?*Obj = null,
    data: Data,

    pub fn print(self: *Obj) void {
        switch (self.data) {
            .string => std.debug.print("{s}", .{self.data.string.chars}),
            .function => self.data.function.print(),
            .native => std.debug.print("<native fn>", .{}),
        }
    }
};

pub const ObjList = struct {
    const StringsSet = Table(void, .{ .cmp_strings = true });

    allocator: std.mem.Allocator,
    head: ?*Obj = null,
    strings: StringsSet,

    pub fn init(allocator: std.mem.Allocator) ObjList {
        return ObjList{
            .allocator = allocator,
            .strings = StringsSet.init(allocator),
        };
    }

    pub fn pop(self: *ObjList) void {
        if (self.head) |obj| {
            self.head = obj.next;

            switch (obj.data) {
                .string => self.allocator.free(obj.data.string.chars),
                .function => obj.data.function.deinit(),
                .native => {},
            }

            self.allocator.destroy(obj);
        }
    }

    pub fn deinit(self: *ObjList) void {
        self.strings.deinit();

        while (self.head) |obj| {
            self.head = obj.next;

            switch (obj.data) {
                .string => self.allocator.free(obj.data.string.chars),
                .function => obj.data.function.deinit(),
                .native => {},
            }

            self.allocator.destroy(obj);
        }
    }

    pub fn new(self: *ObjList) !*Obj {
        var obj = try self.allocator.create(Obj);
        obj.next = self.head;
        self.head = obj;
        return obj;
    }

    pub fn newString(self: *ObjList, chars: []const u8) !*Obj {
        const obj = try self.new();
        const string = Obj.String.init(chars);
        obj.data = Obj.Data{ .string = string };

        const result = try self.strings.getOrPut(obj);
        if (result.found_existing)
            self.pop();

        return result.key_ptr.*;
    }

    pub fn newFunction(self: *ObjList) !*Obj {
        const obj = try self.new();
        const function = Obj.Function.init(self.allocator);
        obj.data = Obj.Data{ .function = function };
        return obj;
    }

    pub fn newNative(self: *ObjList, function: Obj.NativeFn) !*Obj {
        const obj = try self.new();
        obj.data = Obj.Data{ .native = function };
        return obj;
    }
};
