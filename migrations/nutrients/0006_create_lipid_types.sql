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
