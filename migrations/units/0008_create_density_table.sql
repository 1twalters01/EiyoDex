CREATE TABLE IF NOT EXISTS units_density_quantities (
    id BLOB PRIMARY KEY NOT NULL,
    mass_type_id INTEGER NOT NULL,
    volume_type_id INTEGER NOT NULL,
    value REAL NOT NULL,
    FOREIGN KEY (mass_type_id)
        REFERENCES units_mass_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (volume_type_id)
        REFERENCES units_volume_types(id)
        ON DELETE CASCADE,
    CHECK (LENGTH(id) = 16)
);

