CREATE TABLE IF NOT EXISTS nutrients_quantity_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO nutrients_quantity_types (id, name) VALUES
(1, 'macronutrient'),
(2, 'micronutrient'),
(3, 'non_nutrient');
