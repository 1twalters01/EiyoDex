CREATE TABLE IF NOT EXISTS quantity_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO quantity_types (id, name) VALUES
(1, 'macronutrient'),
(2, 'micronutrient'),
(3, 'non_nutrient');
