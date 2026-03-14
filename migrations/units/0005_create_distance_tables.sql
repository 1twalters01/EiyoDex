CREATE TABLE IF NOT EXISTS units_distance_types (
    id INTEGER PRIMARY KEY,
    unit_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS units_distance_quantities (
    id BLOB PRIMARY KEY NOT NULL,
    distance_type_id INTEGER NOT NULL,
    value REAL NOT NULL,
    FOREIGN KEY (distance_type_id)
        REFERENCES units_distance_types(id)
        ON DELETE RESTRICT,
    CHECK (LENGTH(id) = 16)
);
