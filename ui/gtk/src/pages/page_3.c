#include "page_3.h"

void show_page_3(GtkWidget *widget, gpointer data) {
    GtkWindow *window = (GtkWindow *)data;

    // Root container
    int page_child_spacing = 10;
    GtkWidget *page_widget = gtk_box_new(GTK_ORIENTATION_VERTICAL, page_child_spacing);
    GtkBox *page = (GtkBox *)page_widget;
    gtk_widget_set_name(page_widget, "page1");
    gtk_widget_set_hexpand(page_widget, TRUE);
    gtk_widget_set_vexpand(page_widget, TRUE);
    gtk_window_set_child(window, page_widget);

    // Header
    int header_child_spacing = 10;
    GtkWidget *header_widget = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, header_child_spacing);
    GtkBox *header = (GtkBox *)header_widget;
    gtk_widget_set_hexpand(header_widget, 1);
    gtk_box_append(page, header_widget);

    // Header title
    GtkWidget *title_widget = gtk_label_new("Page 3");
    GtkLabel *title = (GtkLabel *)title_widget;
    gtk_widget_set_name(title_widget, "title");
    gtk_widget_set_hexpand(title_widget, TRUE);
    gtk_label_set_xalign(title, 0.5);
    gtk_box_append(header, title_widget);

    // Page Content
    int content_child_spacing = 10;
    GtkWidget *content_widget = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, content_child_spacing);
    GtkBox *content = (GtkBox *)content_widget;
    gtk_widget_set_hexpand(content_widget, TRUE);
    gtk_widget_set_vexpand(content_widget, TRUE);
    gtk_box_append(page, content_widget);

    // Page 1 button
    GtkWidget *page_1_button_widget = gtk_button_new();
    GtkButton *page_1_button = (GtkButton *)page_1_button_widget;
    gtk_widget_set_hexpand(page_1_button_widget, TRUE);
    gtk_button_set_label(
        page_1_button,
        "Page 1 (zig)"
    );

    // Page 2 button
    GtkWidget *page_2_button_widget = gtk_button_new();
    GtkButton *page_2_button = (GtkButton *)page_2_button_widget;
    gtk_widget_set_name(page_2_button_widget, "page3button");
    gtk_widget_set_hexpand(page_2_button_widget, TRUE);
    gtk_button_set_label(
        page_2_button,
        "Page 2 (zig)"
    );

    // Page 4 button
    GtkWidget *page_4_button_widget = gtk_button_new();
    GtkButton *page_4_button = (GtkButton *)page_4_button_widget;
    gtk_widget_set_hexpand(page_4_button_widget, TRUE);
    gtk_button_set_label(
        page_4_button,
        "Page 4 (c)"
    );

    // Connect "clicked" signal to button and function
    g_signal_connect(page_1_button, "clicked", G_CALLBACK(show_page_1), window);
    g_signal_connect(page_2_button, "clicked", G_CALLBACK(show_page_2), window);
    g_signal_connect(page_4_button, "clicked", G_CALLBACK(show_page_4), window);

    // Add buttons to container
    gtk_box_append(content, page_1_button_widget);
    gtk_box_append(content, page_2_button_widget);
    gtk_box_append(content, page_4_button_widget);

    // Load CSS
    GtkCssProvider *provider = gtk_css_provider_new();
    GtkStyleProvider *provider_style = (GtkStyleProvider *)provider;
    gtk_css_provider_load_from_path(provider, "ui/test_styles/page_1.css");

    // Apply CSS globally so it affects children
    GdkDisplay *display = gdk_display_get_default();
    gtk_style_context_add_provider_for_display(
            display,
            provider_style,
        GTK_STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}
