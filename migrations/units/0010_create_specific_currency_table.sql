CREATE TABLE IF NOT EXISTS units_specific_currency_quantities (
    id BLOB PRIMARY KEY NOT NULL,
    currency_type_id INTEGER NOT NULL,
    mass_type_id INTEGER,
    volume_type_id INTEGER,
    value REAL NOT NULL,
    FOREIGN KEY (energy_type_id)
        REFERENCES units_energy_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (duration_type_id)
        REFERENCES units_duration_types(id)
        ON DELETE CASCADE,
    CHECK (LENGTH(id) = 16)
    CHECK (
        (mass_type_id IS NOT NULL) +
        (volume_type_id IS NOT NULL)
        = 1
    )
);

