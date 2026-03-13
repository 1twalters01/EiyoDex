#include "page_3.h"

void show_page_3(GtkWidget *widget, gpointer data) {
    GtkWindow *window = (GtkWindow *)data;

    // Create a new button
    GtkWidget *button_widget = gtk_button_new();
    GtkButton *button = (GtkButton *)button_widget;
    gtk_button_set_label(
        button,
        "This is page 3 (made in C). Go to page 1"
    );

    g_signal_connect(button, "clicked", G_CALLBACK(show_page_1), window);
    gtk_window_set_child(GTK_WINDOW(window), button_widget);
}
