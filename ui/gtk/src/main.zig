const std = @import("std");

const gtk = @import("gtk.zig").gtk_lib;

const activate = @import("activate.zig").activate;


pub fn main() !void {
    const argc: c_int = 0;
    const argv: [*c][*c]u8 = null;

    const app: *gtk.GtkApplication = gtk.gtk_application_new("eiyodex.gtk", 0);
    const app_ptr: *gtk.GApplication = @ptrCast(app);
    defer gtk.g_object_unref(app);

    // Connect "activate" signal
    _ = gtk.g_signal_connect_data(
        app,
        "activate",
        @ptrCast(&activate),
        null,
        null,
        0,
    );

    // Run the application and exit with proper type
    const status: c_int = gtk.g_application_run(app_ptr, argc, argv);
    std.process.exit(@intCast(status));
}
