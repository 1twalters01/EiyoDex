const std = @import("std");
const c = @cImport({
    @cInclude("gtk/gtk.h");
});

// Button clicked callback
fn on_button_clicked(button: ?*c.GtkButton, _: ?*c.gpointer) callconv(.c) void {
    if (button) |b| {
        c.gtk_button_set_label(b, "Clicked!");
    }
}

// Application "activate" callback
fn activate(app: ?*c.GtkApplication, _: ?*c.gpointer) callconv(.c) void {
    if (app) |a| {
        const window_widget = c.gtk_application_window_new(a);
        if (window_widget) |w| {
            const window: *c.GtkWindow = w;

            c.gtk_window_set_title(window, "GTK4 + Zig");
            c.gtk_window_set_default_size(window, 400, 300);

            const button_widget = c.gtk_button_new_with_label("Click Me!");
            if (button_widget) |b| {
                c.gtk_window_set_child(window, b);

                _ = c.g_signal_connect_data(
                    b,
                    "clicked",
                    on_button_clicked,
                    null,
                    null,
                    0,
                );
            }

            c.gtk_window_present(window);
        }
    }
}

pub fn main() !void {
    const argc: c_int = 0;
    const argv: [*c]?[*:0]u8 = null;

    const app = c.gtk_application_new("com.example.gtk4zig", 0) orelse return error.FailedToCreateApp;
    defer c.g_object_unref(app);

    _ = c.g_signal_connect_data(
        app,
        "activate",
        activate,
        null,
        null,
        0,
    );

    std.process.exit(c.g_application_run(app, argc, argv));
}
