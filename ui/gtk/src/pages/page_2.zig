const gtk = @import("../gtk.zig").gtk_lib;
const page_1 = @import("page_1.zig");

pub fn show_page_2(_: *gtk.GtkWidget, data: gtk.gpointer) callconv(.c) void {
    const window: *gtk.GtkWindow = @ptrCast(@alignCast(data));

    // Create a new button
    const button_widget: *gtk.GtkWidget = gtk.gtk_button_new();
    const button: *gtk.GtkButton = @ptrCast(button_widget);
    gtk.gtk_button_set_label(button, "This is page 2. Click to go to page 1");

    // Connect "clicked" signal
    _ = gtk.g_signal_connect_data(
        button,
        "clicked",
        @ptrCast(&page_1.show_page_1),
        window,
        null,
        0,
    );

    gtk.gtk_window_set_child(window, button_widget);
}

