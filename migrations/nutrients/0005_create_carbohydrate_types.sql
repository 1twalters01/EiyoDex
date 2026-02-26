CREATE TABLE IF NOT EXISTS nutrients_carbohydrate_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO nutrients_carbohydrate_types (id, name) VALUES
(1, 'fiber'),
(2, 'starch'),
(3, 'sugar'),
(4, 'sugar_alcohol');
