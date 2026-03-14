CREATE TABLE IF NOT EXISTS units_energy_types (
    id INTEGER PRIMARY KEY,
    unit_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS units_energy_quantities (
    id BLOB PRIMARY KEY NOT NULL,
    energy_type_id INTEGER NOT NULL,
    value REAL NOT NULL,
    FOREIGN KEY (energy_type_id)
        REFERENCES units_energy_types(id)
        ON DELETE RESTRICT,
    CHECK (LENGTH(id) = 16)
);
