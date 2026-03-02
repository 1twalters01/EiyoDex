CREATE TABLE IF NOT EXISTS nutrients_nutrient_lists (
    id INTEGER PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS nutrients_nutrient_list_items (
    id INTEGER PRIMARY KEY,
    nutrient_list_id NOT NULL,
    nutrient_id INTEGER NOT NULL,
    FOREIGN KEY (nutrient_list_id)
        REFERENCES nutrients_nutrient_lists(id)
        ON DELETE CASCADE,
    FOREIGN KEY (output_id)
        REFERENCES nutrients_nutrient_table(id)
        ON DELETE CASCADE
);
