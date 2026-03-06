CREATE TABLE IF NOT EXISTS nutrients_nutrient_quantity_table (
    id BLOB PRIMARY KEY NOT NULL,
    quantity REAL NOT NULL,
    nutrient_id BLOB NOT NULL,
    output_unit_id INTEGER NOT NULL,
    FOREIGN KEY (nutrient_id)
        REFERENCES nutrients_nutrient_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (output_unit_id)
        REFERENCES nutrients_unit_table(id)
        ON DELETE CASCADE
);
