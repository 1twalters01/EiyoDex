CREATE TABLE IF NOT EXISTS nutrients_nutrient_table (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    nutrient_type_id INTEGER NOT NULL,
    main_unit_id INTEGER NOT NULL,
    FOREIGN KEY (nutrient_type_id)
        REFERENCES nutrients_nutrient_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (main_unit_id)
        REFERENCES nutrients_unit_table(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS nutrients_unit_conversions (
    nutrient_id INTEGER NOT NULL,
    unit_id INTEGER NOT NULL,
    factor REAL NOT NULL,
    PRIMARY KEY (nutrient_id, unit_id),
    FOREIGN KEY (nutrient_id)
        REFERENCES nutrients_nutrient_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (unit_id)
        REFERENCES nutrients_unit_table(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS nutrients_nutrient_relationships (
    parent_id INTEGER NOT NULL,
    child_id INTEGER NOT NULL,
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
