const gtk = @cImport({
    @cInclude("gtk/gtk.h");
});

const page_3_c = @cImport({
    @cInclude("pages/page_3.h");
});

const page_4_c = @cImport({
    @cInclude("pages/page_4.h");
});

pub const gtk_lib = gtk;
pub const pages_3 = page_3_c;
pub const pages_4 = page_4_c;
