CREATE TABLE IF NOT EXISTS nutrients_carbohydrate_nutrients (
    id INTEGER PRIMARY KEY NOT NULL,
    carbohydrate_type_id INTEGER UNIQUE NOT NULL,
    FOREIGN KEY(carbohydrate_type_id)
        REFERENCES nutrients_carbohydrate_types (id)
        ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS nutrients_protein_nutrients (
    id INTEGER PRIMARY KEY NOT NULL,
    is_bcaa BOOLEAN UNIQUE NOT NULL
);


CREATE TABLE IF NOT EXISTS nutrients_lipid_nutrients (
    id INTEGER PRIMARY KEY NOT NULL,
    lipid_id INTEGER UNIQUE NOT NULL,
    FOREIGN KEY (lipid_id)
        REFERENCES nutrients_lipid_table(id)
        ON DELETE CASCADE
);
