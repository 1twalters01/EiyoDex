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
    id INTEGER PRIMARY KEY NOT NULL,
    energy_yielding_nutrient_type_id INTEGER NOT NULL,
    carbohydrate_nutrient_id INTEGER,
    protein_nutrient_id INTEGER,
    lipid_nutrient_id INTEGER,
    FOREIGN KEY (energy_yielding_nutrient_type_id)
        REFERENCES nutrients_energy_yielding_nutrient_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (carbohydrate_nutrient_id)
        REFERENCES nutrients_carbohydrate_nutrients(id)
        ON DELETE CASCADE,
    FOREIGN KEY (protein_nutrient_id)
        REFERENCES nutrients_protein_nutrients(id)
        ON DELETE CASCADE,
    FOREIGN KEY (lipid_nutrient_id)
        REFERENCES nutrients_lipid_nutrients(id)
        ON DELETE CASCADE
    CHECK (
        (carbohydrate_nutrient_id IS NOT NULL) +
        (protein_nutrient_id IS NOT NULL) +
        (lipid_nutrient_id IS NOT NULL)
        <= 1
    )
);

CREATE UNIQUE INDEX nutrients_unique_energy_yielding_type_for_alcohol
ON nutrients_energy_yielding_nutrients(energy_yielding_nutrient_type_id)
WHERE energy_yielding_nutrient_type_id != 4;

