CREATE TABLE IF NOT EXISTS units_duration_types (
    id INTEGER PRIMARY KEY,
    unit_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS units_duration_quantities (
    id BLOB PRIMARY KEY NOT NULL,
    duration_type_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    FOREIGN KEY (duration_type_id)
        REFERENCES units_duration_types(id)
        ON DELETE CASCADE,
    CHECK (LENGTH(id) = 16)
);
