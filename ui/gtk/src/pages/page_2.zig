const gtk = @import("../c.zig").gtk_lib;
const page_3 = @import("../c.zig").pages_3;

pub fn show_page_2(_: *gtk.GtkWidget, data: gtk.gpointer) callconv(.c) void {
    const window: *gtk.GtkWindow = @ptrCast(@alignCast(data));

    // Create a new button
    const button_widget: *gtk.GtkWidget = gtk.gtk_button_new();
    const button: *gtk.GtkButton = @ptrCast(button_widget);
    gtk.gtk_button_set_label(
        button,
        "This is page 2 (zig).\nClick to go to page 3 (c)"
    );

    // Connect "clicked" signal
    _ = gtk.g_signal_connect_data(
        button,
        "clicked",
        @ptrCast(&page_3.show_page_3),
        window,
        null,
        0,
    );

    gtk.gtk_window_set_child(window, button_widget);
}
