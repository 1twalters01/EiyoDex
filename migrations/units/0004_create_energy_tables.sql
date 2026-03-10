CREATE TABLE IF NOT EXISTS units_energy_types (
    id INTEGER PRIMARY KEY,
    unit_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS units_energy_quantities (
    id INTEGER PRIMARY KEY,
    energy_type_id INTEGER NOT NULL,
    quantity REAL NOT NULL,
    FOREIGN KEY (energy_type_id)
        REFERENCES units_energy_types(id)
        ON DELETE CASCADE
);
