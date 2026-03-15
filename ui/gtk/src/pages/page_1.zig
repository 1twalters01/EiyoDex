const gtk = @import("../c.zig").gtk_lib;
const page_2 = @import("page_2.zig");
const page_3 = @import("../c.zig").pages_3;
const page_4 = @import("../c.zig").pages_4;

pub export fn show_page_1(_: *gtk.GtkWidget, data: gtk.gpointer) callconv(.c) void {
    const window: *gtk.GtkWindow = @ptrCast(@alignCast(data));

    // Root container
    const page_child_spacing: c_int = 10;
    const page_widget: *gtk.GtkWidget = gtk.gtk_box_new(gtk.GTK_ORIENTATION_VERTICAL, page_child_spacing);
    const page: *gtk.GtkBox = @ptrCast(page_widget);
    gtk.gtk_widget_set_name(page_widget, "page1"); // set css id
    gtk.gtk_widget_set_hexpand(page_widget, 1); // horizontally expand page widget as much as possible
    gtk.gtk_widget_set_vexpand(page_widget, 1);
    gtk.gtk_window_set_child(window, page_widget);

    // Header
    const header_child_spacing: c_int = 10;
    const header_widget: *gtk.GtkWidget = gtk.gtk_box_new(gtk.GTK_ORIENTATION_HORIZONTAL, header_child_spacing);
    const header: *gtk.GtkBox = @ptrCast(header_widget);
    gtk.gtk_widget_set_hexpand(header_widget, 1);
    gtk.gtk_box_append(page, header_widget);
    
    // Header title
    const title_widget: *gtk.GtkWidget = gtk.gtk_label_new("Page 1");
    const title: *gtk.GtkLabel = @ptrCast(title_widget);
    // gtk.gtk_label_set_selectable(title, 1); // Can select the text in the title
    gtk.gtk_widget_set_name(title_widget, "title");
    gtk.gtk_widget_set_hexpand(title_widget, 1);
    gtk.gtk_label_set_xalign(title, 0.5);
    gtk.gtk_box_append(header, title_widget);

    // Page Content
    const child_spacing: c_int = 10;
    const content_widget: *gtk.GtkWidget = gtk.gtk_box_new(gtk.GTK_ORIENTATION_HORIZONTAL, child_spacing);
    const content: *gtk.GtkBox = @ptrCast(content_widget);
    gtk.gtk_widget_set_hexpand(content_widget, 1);
    gtk.gtk_widget_set_vexpand(content_widget, 1);
    gtk.gtk_box_append(page, content_widget);

    // Page 2 button
    const page_2_button_widget: *gtk.GtkWidget = gtk.gtk_button_new();
    const page_2_button: *gtk.GtkButton = @ptrCast(page_2_button_widget);
    gtk.gtk_widget_set_hexpand(page_2_button_widget, 1);
    gtk.gtk_button_set_label(
        page_2_button,
        "Page 2 (zig)"
    );

    // Page 3 button
    const page_3_button_widget: *gtk.GtkWidget = gtk.gtk_button_new();
    const page_3_button: *gtk.GtkButton = @ptrCast(page_3_button_widget);
    gtk.gtk_widget_set_name(page_3_button_widget, "page3button");
    gtk.gtk_widget_set_hexpand(page_3_button_widget, 1);
    gtk.gtk_button_set_label(
        page_3_button,
        "Page 3 (zig)"
    );


    // Page 4 button
    const page_4_button_widget: *gtk.GtkWidget = gtk.gtk_button_new();
    const page_4_button: *gtk.GtkButton = @ptrCast(page_4_button_widget);
    gtk.gtk_widget_set_hexpand(page_4_button_widget, 1);
    gtk.gtk_button_set_label(
        page_4_button,
        "Page 4 (zig)"
    );

    // Connect "clicked" signal
    _ = gtk.g_signal_connect_data(
        page_2_button,
        "clicked",
        @ptrCast(&page_2.show_page_2),
        window,
        null,
        0,
    );

    _ = gtk.g_signal_connect_data(
        page_3_button,
        "clicked",
        @ptrCast(&page_3.show_page_3),
        window,
        null,
        0,
    );

    _ = gtk.g_signal_connect_data(
        page_4_button,
        "clicked",
        @ptrCast(&page_4.show_page_4),
        window,
        null,
        0,
    );

    // Add page_2_button to container
    gtk.gtk_box_append(content, page_2_button_widget);
    gtk.gtk_box_append(content, page_3_button_widget);
    gtk.gtk_box_append(content, page_4_button_widget);

    // Load CSS
    const provider = gtk.gtk_css_provider_new();
    _ = gtk.gtk_css_provider_load_from_path(provider, "ui/test_styles/page_1.css");

    // Apply CSS globally so it affects children
    const display = gtk.gdk_display_get_default();
    gtk.gtk_style_context_add_provider_for_display(
        display,
        @ptrCast(provider),
        gtk.GTK_STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}
