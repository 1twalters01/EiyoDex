CREATE TABLE IF NOT EXISTS units_duration_types (
    id INTEGER PRIMARY KEY,
    unit_type TEXT NOT NULL
    -- measurement_system_id INTEGER NOT NULL,
    -- FOREIGN KEY (measurement_system_id)
    --     REFERENCES units_measurement_systems(id)
    --     ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS units_duration_quantities (
    id INTEGER PRIMARY KEY,
    duration_type_id INTEGER NOT NULL,
    quantity REAL NOT NULL,
    FOREIGN KEY (duration_type_id)
        REFERENCES units_duration_types(id)
        ON DELETE CASCADE
);
