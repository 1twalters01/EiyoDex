CREATE TABLE IF NOT EXISTS nutrients_carbohydrate_nutrients (
    id INTEGER PRIMARY KEY,
    carbohydrate_id INTEGER UNIQUE NOT NULL,
    FOREIGN KEY(carbohydrate_id)
        REFERENCES nutrients_carbohydrate_types (id)
        ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS nutrients_protein_nutrients (
    id INTEGER PRIMARY KEY,
    is_bcaa BOOLEAN UNIQUE NOT NULL
);


CREATE TABLE IF NOT EXISTS nutrients_lipid_nutrients (
    id INTEGER PRIMARY KEY,
    lipid_id INTEGER UNIQUE NOT NULL,
    FOREIGN KEY (lipid_id)
        REFERENCES nutrients_lipid_table(id)
        ON DELETE CASCADE
);
