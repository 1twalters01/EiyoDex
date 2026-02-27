CREATE TABLE IF NOT EXISTS units_currency_types (
    id INTEGER PRIMARY KEY,
    symbol TEXT NOT NULL,
    code TEXT not NULL,
    unit_type TEXT NOT NULL,
    unit_type_plural TEXT NOT NULL,
    FOREIGN KEY (measurement_system_id)
        REFERENCES units_measurement_systems(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS units_currency_quantities (
    id INTEGER PRIMARY KEY,
    mass_type_id INTEGER NOT NULL,
    quantity REAL NOT NULL,
    FOREIGN KEY (mass_type_id)
        REFERENCES units_currency_types(id)
        ON DELETE CASCADE
);
