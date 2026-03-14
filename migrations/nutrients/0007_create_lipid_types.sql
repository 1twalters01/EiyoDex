CREATE TABLE IF NOT EXISTS nutrients_lipid_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO nutrients_lipid_types (id, name) VALUES
(1, 'sterol'),
(2, 'fat'),
(3, 'transfat'),
(4, 'phospholipid');

CREATE TABLE IF NOT EXISTS nutrients_lipid_table (
    id INTEGER PRIMARY KEY,
    lipid_type_id INTEGER NOT NULL,
    sterol_type_id INTEGER UNIQUE,
    fat_type_id INTEGER UNIQUE,
    transfat_type_id INTEGER UNIQUE,
    FOREIGN KEY (lipid_type_id)
        REFERENCES nutrients_lipid_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (sterol_type_id)
        REFERENCES nutrients_sterol_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (fat_type_id)
        REFERENCES nutrients_fat_types(id)
        ON DELETE CASCADE,
    FOREIGN KEY (transfat_type_id)
        REFERENCES nutrients_transfat_types(id)
        ON DELETE CASCADE,
    -- at most have one subtype
    CHECK (
        (sterol_type_id IS NOT NULL) +
        (fat_type_id IS NOT NULL) +
        (transfat_type_id IS NOT NULL)
        <= 1
    ),
    -- lipid is phospholipid if no subtypes
    CHECK (
        ( (sterol_type_id IS NOT NULL) +
        (fat_type_id IS NOT NULL) +
        (transfat_type_id IS NOT NULL)
        ) != 0
        OR lipid_type_id = 4
    ),
    -- lipid is not phospholipid if there are subtypes
    CHECK (
        ( (sterol_type_id IS NOT NULL) +
        (fat_type_id IS NOT NULL) +
        (transfat_type_id IS NOT NULL)
        ) != 1
        OR lipid_type_id != 4
    )
);


CREATE TABLE IF NOT EXISTS nutrients_sterol_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO nutrients_sterol_types (id, name) VALUES
(1, 'cholesterol'),
(2, 'phytosterol');


CREATE TABLE IF NOT EXISTS nutrients_fat_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO nutrients_fat_types (id, name) VALUES
(1, 'monounsaturated'),
(2, 'polyunsaturated'),
(3, 'saturated');


CREATE TABLE IF NOT EXISTS nutrients_transfat_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO nutrients_transfat_types (id, name) VALUES
(1, 'natural'),
(2, 'artificial');
