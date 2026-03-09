CREATE TABLE IF NOT EXISTS foods_recipe_lists (
    id INTEGER PRIMARY KEY,
);

CREATE TABLE IF NOT EXISTS foods_recipe_items (
    id INTEGER PRIMARY KEY,
    recipe_list_id NOT NULL,
    food_quantity_id INTEGER NOT NULL,
    FOREIGN KEY (recipe_list_id)
        REFERENCES foods_recipe_lists(id)
        ON DELETE CASCADE,
    FOREIGN KEY (food_quantity_id)
        REFERENCES foods_food_quantity_table(id)
        ON DELETE CASCADE
);
