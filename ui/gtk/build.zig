const std = @import("std");

pub fn build(b: *std.Build) void {
    // Get target/optimize from command line
    // (e.g. `zig build -Doptimize=ReleaseFast`)
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create module
    const module = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Create executable with main.zig as entry point
    const exe = b.addExecutable(.{
        .name = "eiyodex-gtk-ui",
        .root_module = module,
    });

    // Add c pages
    exe.root_module.addIncludePath(b.path("src"));
    exe.root_module.addCSourceFiles(.{
        .files = &[_][]const u8{
            "src/pages/page_3.c",
        },
    });

    // Apply c links/macros to module
    exe.root_module.link_libc = true;
    exe.root_module.linkSystemLibrary("gtk4", .{
        .use_pkg_config = .yes,
    });
    exe.root_module.addCMacro("G_DISABLE_DEPRECATED", "1");
    exe.root_module.addCMacro("GTK_DISABLE_DEPRECATED", "1");
    exe.root_module.addCMacro("G_LOG_USE_STRUCTURED", "1");



    // Install executable to zig-out/bin/
    b.installArtifact(exe);

    // Create run command (`zig build run`)
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep()); // Build before running

    // Forward command-line args to the executable
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    // Register "run" step
    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
