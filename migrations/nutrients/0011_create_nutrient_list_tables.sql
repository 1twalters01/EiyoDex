CREATE TABLE IF NOT EXISTS nutrients_nutrient_list_table (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS nutrients_nutrient_list_items (
    nutrient_list_id BLOB NOT NULL,
    nutrient_id BLOB NOT NULL,
    PRIMARY KEY (nutrient_list_id, nutrient_id),
    FOREIGN KEY (nutrient_list_id)
        REFERENCES nutrients_nutrient_list_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (nutrient_id)
        REFERENCES nutrients_nutrient_table(id)
        ON DELETE CASCADE
);
