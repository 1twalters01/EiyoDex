CREATE TABLE IF NOT EXISTS nutrients_nutrient_quantity (
    id INTEGER PRIMARY KEY,
    quantity REAL NOT NULL,
    nutrient_id INTEGER NOT NULL UNIQUE,
    output_unit_id INTEGER NOT NULL UNIQUE,
    FOREIGN KEY (nutrient_id)
        REFERENCES nutrients_nutrient_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (output_unit_id)
        REFERENCES nutrients_unit_table(id)
        ON DELETE CASCADE
);
