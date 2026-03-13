const std = @import("std");

const gtk = @cImport({
    @cInclude("gtk/gtk.h");
});

// Button clicked callback
fn on_button_clicked(button: *gtk.GtkButton, _: ?*gtk.gpointer) callconv(.c) void {
    gtk.gtk_button_set_label(button, "Clicked!");
}

// Application "activate" callback
fn activate(app: *gtk.GtkApplication, _: ?*gtk.gpointer) callconv(.c) void {
    // Create a new window
    const window_widget: *gtk.GtkWidget = gtk.gtk_application_window_new(app);
    const window: *gtk.GtkWindow = @ptrCast(window_widget);
    gtk.gtk_window_set_title(window, "GTK4 + Zig");
    gtk.gtk_window_set_default_size(window, 400, 300);

    // Create a new button
    const button_widget: *gtk.GtkWidget = gtk.gtk_button_new();
    const button: *gtk.GtkButton = @ptrCast(button_widget);
    gtk.gtk_button_set_label(button, "Press here!");

    // Add button to window
    gtk.gtk_window_set_child(window, button_widget);

    // Connect "clicked" signal
    _ = gtk.g_signal_connect_data(
        button,
        "clicked",
        @ptrCast(&on_button_clicked),
        null,
        null,
        0,
    );

    // Show the window
    gtk.gtk_window_present(window);
}

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
