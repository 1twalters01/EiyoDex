const c = @cImport({
    @cInclude("gtk/gtk.h");
    // @cInclude("page_3.h");
});

pub const gtk_lib = c;
