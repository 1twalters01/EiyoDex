CREATE TABLE IF NOT EXISTS units_distance_types (
    id INTEGER PRIMARY KEY,
    unit_type TEXT NOT NULL
    -- measurement_system_id INTEGER NOT NULL,
    -- FOREIGN KEY (measurement_system_id)
    --     REFERENCES units_measurement_systems(id)
    --     ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS units_distance_quantities (
    id INTEGER PRIMARY KEY,
    distance_type_id INTEGER NOT NULL,
    quantity REAL NOT NULL,
    FOREIGN KEY (distance_type_id)
        REFERENCES units_distance_types(id)
        ON DELETE CASCADE
);
