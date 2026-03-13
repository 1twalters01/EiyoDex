const gtk = @import("c.zig").gtk_lib;
const page_1 = @import("pages/page_1.zig");

    
// Application "activate" callback
pub fn activate(app: *gtk.GtkApplication, _: gtk.gpointer) callconv(.c) void {
    // Create a new window
    const window_widget: *gtk.GtkWidget = gtk.gtk_application_window_new(app);
    const window: *gtk.GtkWindow = @ptrCast(@alignCast(window_widget));
    gtk.gtk_window_set_title(window, "GTK4 + Zig");
    gtk.gtk_window_set_default_size(window, 400, 300);

    page_1.show_page_1(window_widget, @ptrCast(window));

    // Show the window
    gtk.gtk_window_present(window);
}
