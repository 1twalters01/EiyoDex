CREATE TABLE IF NOT EXISTS nutrients_unit_types (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO nutrients_unit_types (id, name) VALUES
(1, 'mass'),
(2, 'volume'),
(3, 'energy'),
(4, 'iu'),
(5, 'dfe'),
(6, 'ne'),
(7, 'rae'),
(8, 'pdcaas'),
(9, 'diaas1'),
(10, 'diaas2'),
(11, 'diaas3');

CREATE TABLE IF NOT EXISTS nutrients_nutrient_units (
    id INTEGER PRIMARY KEY NOT NULL,
    unit_type_id INTEGER NOT NULL,
    mass_type_id INTEGER UNIQUE,
    volume_type_id INTEGER UNIQUE,
    energy_type_id INTEGER UNIQUE,

    FOREIGN KEY (unit_type_id)
        REFERENCES nutrients_unit_types(id)
        ON DELETE RESTRICT,

    FOREIGN KEY (mass_type_id)
        REFERENCES units_mass_types(id)
        ON DELETE RESTRICT,

    FOREIGN KEY (volume_type_id)
        REFERENCES units_volume_types(id)
        ON DELETE RESTRICT,

    FOREIGN KEY (energy_type_id)
        REFERENCES units_energy_types(id)
        ON DELETE RESTRICT,

    CHECK (
        (mass_type_id IS NOT NULL) +
        (volume_type_id IS NOT NULL) +
        (energy_type_id IS NOT NULL)
        <= 1
    ),

    CHECK (
        (mass_type_id IS NOT NULL AND unit_type_id = 1) OR
        (volume_type_id IS NOT NULL AND unit_type_id = 2) OR
        (energy_type_id IS NOT NULL AND unit_type_id = 3) OR
        (unit_type_id > 3)
    )
);
