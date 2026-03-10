CREATE TABLE IF NOT EXISTS units_volume_types (
    id INTEGER PRIMARY KEY,
    unit_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS units_volume_quantities (
    id INTEGER PRIMARY KEY,
    volume_type_id INTEGER NOT NULL,
    quantity REAL NOT NULL,
    FOREIGN KEY (volume_type_id)
        REFERENCES units_volume_types(id)
        ON DELETE CASCADE
);
