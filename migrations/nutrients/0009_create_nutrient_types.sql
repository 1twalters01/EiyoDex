CREATE TABLE IF NOT EXISTS nutrients_nutrient_types (
    essentiality_type_id INTEGER NOT NULL,
    quantity_type_id INTEGER NOT NULL,
    chemical_id INTEGER NOT NULL,
    PRIMARY KEY (essentiality_type_id, quantity_type_id, chemical_id),
    FOREIGN KEY (quantity_type_id)
        REFERENCES nutrients_quantity_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (essentiality_type_id)
        REFERENCES nutrients_essentiality_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (chemical_id)
        REFERENCES nutrients_chemical_type_table(id)
        ON DELETE CASCADE
);

