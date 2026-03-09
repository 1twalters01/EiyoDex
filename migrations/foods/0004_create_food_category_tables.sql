CREATE TABLE IF NOT EXISTS foods_food_categories (
    id BLOB PRIMARY KEY NOT NULL CHECK(length(id) = 16),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
}

CREATE TABLE IF NOT EXISTS foods_food_taxonomies (
    id BLOB PRIMARY KEY NOT NULL CHECK(length(id) = 16),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
}

CREATE TABLE IF NOT EXISTS foods_food_category_relationships (
    parent_id BLOB NOT NULL,
    child_category_id BLOB,
    child_taxonomy_id BLOB,
    PRIMARY KEY (parent_id, child_category_id, child_taxonomy_id),
    FOREIGN KEY (parent_id)
        REFERENCES foods_food_categories(id)
        ON DELETE CASCADE,
    FOREIGN KEY (child_category_id)
        REFERENCES foods_food_categories(id)
        ON DELETE CASCADE,
    FOREIGN KEY (child_taxonomy_id)
        REFERENCES foods_food_taxonomies(id)
        ON DELETE CASCADE,
    CHECK (
        (child_category_id IS NOT NULL) +
        (child_taxonomy_id IS NOT NULL)
        = 1
    )
)

CREATE INDEX idx_food_category_parent
ON foods_food_category_relationships(parent_id);

CREATE INDEX idx_food_category_child_category
ON foods_food_category_relationships(child_category_id);

CREATE INDEX idx_food_category_child_taxonomy
ON foods_food_category_relationships(child_taxonomy_id);
