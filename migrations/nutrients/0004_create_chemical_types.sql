CREATE TABLE IF NOT EXISTS nutrients_chemical_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO nutrients_chemical_types (id, name) VALUES
(1, 'energy'),
(2, 'water'),
(3, 'vitamin'),
(4, 'mineral'),
(5, 'phytonutrient'),
(6, 'antinutrient'),
(7, 'other');

CREATE TABLE IF NOT EXISTS nutrients_chemical_type_table (
    id INTEGER PRIMARY KEY,
    chemical_type_id INTEGER NOT NULL,
    energy_yielding_nutrient_id INTEGER,
    FOREIGN KEY (chemical_type_id)
        REFERENCES nutrients_chemical_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (energy_yielding_nutrient_id)
        REFERENCES nutrients_energy_yielding_nutrients(id)
        ON DELETE CASCADE,
    CHECK (
        chemical_type_id != 1
        OR energy_yielding_nutrient_id IS NOT NULL
    )
);
