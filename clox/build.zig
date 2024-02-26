const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const exe = b.addExecutable(.{
        .name = "clox",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Project flags
    const debug_trace_execution = b.option(bool, "debug_trace_execution", "Disassemble the bytecode instruction when executing.") orelse false;
    const debug_print_code = b.option(bool, "debug_print_code", "Disassemble the chunk's bytecode after compiling.") orelse false;
    const debug = b.option(bool, "debug", "Enable debug features.") orelse false;

    const options = b.addOptions();
    options.addOption(bool, "debug_trace_execution", debug_trace_execution or debug);
    options.addOption(bool, "debug_print_code", debug_print_code or debug);
    exe.addOptions("flags", options);

    b.installArtifact(exe);

    // Run command
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
