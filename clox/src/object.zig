const std = @import("std");

pub const Obj = struct {
    const Data = union(enum) {
        string: []const u8,
    };

    next: ?*Obj = null,
    data: Data,

    pub fn print(self: *Obj) void {
        switch (self.data) {
            .string => std.debug.print("{s}", .{self.data.string}),
        }
    }
};

pub const ObjList = struct {
    allocator: std.mem.Allocator,
    head: ?*Obj = null,

    pub fn init(allocator: std.mem.Allocator) ObjList {
        return ObjList{ .allocator = allocator };
    }

    pub fn deinit(self: *ObjList) void {
        while (self.head) |obj| {
            self.head = obj.next;

            switch (obj.data) {
                .string => self.allocator.free(obj.data.string),
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
};
