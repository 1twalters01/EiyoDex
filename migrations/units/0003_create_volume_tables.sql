CREATE TABLE IF NOT EXISTS units_volume_types (
    id INTEGER PRIMARY KEY,
    identifier TEXT NOT NULL,
    symbol TEXT not NULL,
    unit_type TEXT NOT NULL,
    unit_type_plural TEXT NOT NULL,
    measurement_system_id INTEGER NOT NULL,
    si_factor REAL NOT NULL
    FOREIGN KEY (measurement_system_id)
        REFERENCES units_measurement_systems(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS units_volume_quantities (
    id INTEGER PRIMARY KEY,
    volume_type_id INTEGER NOT NULL,
    quantity REAL NOT NULL,
    FOREIGN KEY (volume_type_id)
        REFERENCES units_volume_types(id)
        ON DELETE CASCADE
);
