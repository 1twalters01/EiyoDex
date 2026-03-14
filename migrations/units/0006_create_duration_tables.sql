CREATE TABLE IF NOT EXISTS units_duration_types (
    id INTEGER PRIMARY KEY,
    unit_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS units_duration_quantities (
    id BLOB PRIMARY KEY NOT NULL,
    duration_type_id INTEGER NOT NULL,
    value REAL NOT NULL,
    FOREIGN KEY (duration_type_id)
        REFERENCES units_duration_types(id)
        ON DELETE RESTRICT,
    CHECK (LENGTH(id) = 16)
);
