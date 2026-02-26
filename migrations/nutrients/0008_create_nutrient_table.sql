CREATE TABLE IF NOT EXISTS nutrients_nutrients (
    chemical_id INTEGER PRIMARY KEY,
    quantity_id INTEGER NOT NULL,
    essentiality_id INTEGER,
    FOREIGN KEY (chemical_id)
        REFERENCES nutrients_chemical_type_kinds(id)
        ON DELETE CASCADE
    FOREIGN KEY (quantity_id)
        REFERENCES nutrients_quantity_type_kinds(id)
        ON DELETE CASCADE
    FOREIGN KEY (essentiality_id)
        REFERENCES nutrients_essentiality_type_kinds(id)
        ON DELETE CASCADE
);

