CREATE TABLE IF NOT EXISTS nutrients_energy_yielding_nutrient_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);
INSERT OR IGNORE INTO nutrients_energy_yielding_nutrient_types (id, name) VALUES
(1, 'carbohydrate'),
(2, 'protein'),
(3, 'lipid'),
(4, 'alcohol');

CREATE TABLE IF NOT EXISTS nutrients_energy_yielding_nutrients (
    chemical_id INTEGER PRIMARY KEY,
    kind_id INTEGER NOT NULL,
    FOREIGN KEY (chemical_id)
        REFERENCES nutrients_chemical_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (kind_id)
        REFERENCES nutrients_energy_yielding_nutrient_types(id)
        ON DELETE CASCADE
);
