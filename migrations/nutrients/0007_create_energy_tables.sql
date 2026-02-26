CREATE TABLE IF NOT EXISTS nutrients_carbohydrate_nutrients (
    energy_id INTEGER PRIMARY KEY,
    carbohydrate_id INTEGER NOT NULL,
    is_added_sugar BOOLEAN NOT NULL,
    glycemic_index INTEGER,
    FOREIGN KEY (energy_id)
        REFERENCES nutrients_energy_yielding_nutrients(chemical_id)
        ON DELETE CASCADE,
    FOREIGN KEY(carbohydrate_id)
        REFERENCES nutrients_carbohydrate_types (id)
        ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS nutrients_protein_nutrients (
    energy_id INTEGER PRIMARY KEY,
    is_bcaa BOOLEAN NOT NULL,
    FOREIGN KEY (energy_id)
        REFERENCES nutrients_energy_yielding_nutrients(chemical_id)
        ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS nutrients_lipid_nutrients (
    energy_id INTEGER PRIMARY KEY,
    lipid_id INTEGER NOT NULL,
    FOREIGN KEY (energy_id)
        REFERENCES nutrients_energy_yielding_nutrients(chemical_id)
        ON DELETE CASCADE,
    FOREIGN KEY(lipid_id)
        REFERENCES nutrients_lipid_types (id)
        ON DELETE CASCADE
);
