CREATE TABLE IF NOT EXISTS nutrients_nutrients (
    id BLOB PRIMARY KEY NOT NULL,
    chemical_id INTEGER NOT NULL,
    quantity_id INTEGER NOT NULL,
    essentiality_id INTEGER,
    FOREIGN KEY (chemical_id)
        REFERENCES nutrients_chemical_types(id)
        ON DELETE CASCADE
    FOREIGN KEY (quantity_id)
        REFERENCES nutrients_quantity_types(id)
        ON DELETE CASCADE
    FOREIGN KEY (essentiality_id)
        REFERENCES nutrients_essentiality_types(id)
        ON DELETE CASCADE
);

