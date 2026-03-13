#include "page_4.h"

extern void show_page_1(GtkWidget *widget, gpointer data);

void show_page_4(GtkWidget *widget, gpointer data) {
    GtkWindow *window = (GtkWindow *)data;

    // Create a new button
    GtkWidget *button_widget = gtk_button_new();
    GtkButton *button = (GtkButton *)button_widget;
    gtk_button_set_label(
        button,
        "This is page 4 (c).\nClick to go to page 1 (zig)"
    );

    g_signal_connect(button, "clicked", G_CALLBACK(show_page_1), window);
    gtk_window_set_child(window, button_widget);
}
