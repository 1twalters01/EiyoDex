CREATE TABLE IF NOT EXISTS units_mass_types (
    id INTEGER PRIMARY KEY,
    unit_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS units_mass_quantities (
    id INTEGER PRIMARY KEY,
    mass_type_id INTEGER NOT NULL,
    quantity REAL NOT NULL,
    FOREIGN KEY (mass_type_id)
        REFERENCES units_mass_types(id)
        ON DELETE CASCADE
);
