const gtk = @cImport({
    @cInclude("gtk/gtk.h");
});

const pages = @cImport({
    @cInclude("pages/page_3.h");
});

pub const gtk_lib = gtk;
pub const pages_c = pages;
