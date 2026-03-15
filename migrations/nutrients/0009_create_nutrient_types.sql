CREATE TABLE IF NOT EXISTS nutrients_nutrient_types (
    essentiality_type_id INTEGER,
    quantity_type_id INTEGER NOT NULL,
    chemical_id INTEGER UNIQUE NOT NULL,
    PRIMARY KEY (quantity_type_id, essentiality_type_id, chemical_id),
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

