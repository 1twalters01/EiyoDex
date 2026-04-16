CREATE TABLE IF NOT EXISTS nutrients_nutrient_quantity_list_table (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS nutrients_nutrient_quantity_list_items (
    nutrient_quantity_list_id BLOB NOT NULL,
    nutrient_quantity_id BLOB NOT NULL,
    PRIMARY KEY (nutrient_quantity_list_id, nutrient_quantity_id),
    FOREIGN KEY (nutrient_quantity_list_id)
        REFERENCES nutrients_nutrient_quantity_list_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (nutrient_quantity_id)
        REFERENCES nutrients_nutrient_quantity_table(id)
        ON DELETE CASCADE
);
