CREATE TABLE IF NOT EXISTS nutrients_nutrient_table (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    main_unit_id INTEGER NOT NULL,

    quantity_type_id INTEGER NOT NULL,
    essentiality_type_id INTEGER NOT NULL,
    chemical_id INTEGER UNIQUE NOT NULL,
    FOREIGN KEY (main_unit_id)
        REFERENCES nutrients_nutrient_units(id)
        ON DELETE CASCADE,
    FOREIGN KEY (quantity_type_id, essentiality_type_id, chemical_id)
        REFERENCES nutrients_nutrient_types (quantity_type_id, essentiality_type_id, chemical_id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS nutrients_unit_conversions (
    nutrient_id BLOB NOT NULL,
    unit_id INTEGER NOT NULL,
    factor REAL NOT NULL,
    PRIMARY KEY (nutrient_id, unit_id),
    FOREIGN KEY (nutrient_id)
        REFERENCES nutrients_nutrient_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (unit_id)
        REFERENCES nutrients_nutrient_units(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS nutrients_nutrient_relationships (
    parent_id BLOB NOT NULL,
    child_id BLOB NOT NULL,
    PRIMARY KEY (parent_id, child_id),
    FOREIGN KEY (parent_id)
        REFERENCES nutrients_nutrient_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (child_id)
        REFERENCES nutrients_nutrient_table(id)
        ON DELETE CASCADE,
    CHECK (
        parent_id != child_id
    )
);
