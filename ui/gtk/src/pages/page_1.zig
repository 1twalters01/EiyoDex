const gtk = @import("../gtk.zig").gtk_lib;
const page_2 = @import("page_2.zig");

pub fn show_page_1(_: *gtk.GtkWidget, data: *gtk.gpointer) callconv(.c) void {
    const window: *gtk.GtkWindow = @ptrCast(data);

    // Create a new button
    const button_widget: *gtk.GtkWidget = gtk.gtk_button_new();
    const button: *gtk.GtkButton = @ptrCast(button_widget);
    gtk.gtk_button_set_label(button, "This is page 1.\nClick to go to page 2");

    // Connect "clicked" signal
    _ = gtk.g_signal_connect_data(
        button,
        "clicked",
        @ptrCast(&page_2.show_page_2),
        window,
        null,
        0,
    );

    gtk.gtk_window_set_child(window, button_widget);
}

