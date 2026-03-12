const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "gtk-ui",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    // Include paths (system headers must use cwd_relative)
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/gtk-4.0" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/pango-1.0" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/fribidi" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/harfbuzz" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/gdk-pixbuf-2.0" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/glycin-2" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/cairo" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/freetype2" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/libpng16" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/pixman-1" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/graphene-1.0" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/lib/graphene-1.0/include" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/glib-2.0" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/lib/glib-2.0/include" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/libmount" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/blkid" });
    exe.addIncludePath(.{ .cwd_relative = "/usr/include/sysprof-6" });

    // Link libraries
    exe.linkSystemLibrary("gtk-4");
    exe.linkSystemLibrary("pangocairo-1.0");
    exe.linkSystemLibrary("pango-1.0");
    exe.linkSystemLibrary("harfbuzz");
    exe.linkSystemLibrary("gdk_pixbuf-2.0");
    exe.linkSystemLibrary("cairo-gobject");
    exe.linkSystemLibrary("cairo");
    exe.linkSystemLibrary("vulkan");
    exe.linkSystemLibrary("graphene-1.0");
    exe.linkSystemLibrary("gio-2.0");
    exe.linkSystemLibrary("gobject-2.0");
    exe.linkSystemLibrary("glib-2.0");

    exe.linkLibC();

    b.installArtifact(exe);
}
