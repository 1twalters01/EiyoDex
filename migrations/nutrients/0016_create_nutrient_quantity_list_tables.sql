CREATE TABLE IF NOT EXISTS nutrients_nutrient_quantity_lists (
    id INTEGER PRIMARY KEY,
);

CREATE TABLE IF NOT EXISTS nutrients_nutrient_list_items (
    id INTEGER PRIMARY KEY,
    nutrient_quantity_list_id NOT NULL,
    nutrient_quantity_id INTEGER NOT NULL,
    FOREIGN KEY (nutrient_quantity_list_id)
        REFERENCES nutrients_nutrient_quantity_lists(id)
        ON DELETE CASCADE,
    FOREIGN KEY (output_id)
        REFERENCES nutrients_nutrient_quantity_table(id)
        ON DELETE CASCADE
);
