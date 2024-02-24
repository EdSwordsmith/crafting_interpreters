const std = @import("std");

const Table = @import("table.zig").Table;
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

    const Data = union(enum) {
        string: String,
    };

    next: ?*Obj = null,
    data: Data,

    pub fn print(self: *Obj) void {
        switch (self.data) {
            .string => std.debug.print("{s}", .{self.data.string.chars}),
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
        obj.data.string = Obj.String.init(chars);

        const result = try self.strings.getOrPut(Value.obj(obj));
        if (result.found_existing)
            self.pop();

        return result.key_ptr.obj;
    }
};
