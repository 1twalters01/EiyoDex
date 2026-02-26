CREATE TABLE IF NOT EXISTS nutrients_chemical_type_kinds (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO nutrients_chemical_type_kinds (id, name) VALUES
(1, 'energy'),
(2, 'water'),
(3, 'vitamin'),
(4, 'mineral'),
(5, 'phytonutrient'),
(6, 'antinutrient'),
(7, 'other');

CREATE TABLE IF NOT EXISTS nutrients_chemical_types (
    id INTEGER PRIMARY KEY,
    kind_id INTEGER NOT NULL,
    FOREIGN KEY (kind_id)
        REFERENCES nutrients_chemical_type_kinds(id)
        ON DELETE CASCADE
);
