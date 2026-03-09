CREATE TABLE IF NOT EXISTS foods_food_variants (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    description NOT NULL,
    parent_id BLOB NOT NULL,
    preparation_method_id BLOB NOT NULL,
    FOREIGN KEY (parent_id)
        REFERENCES foods_food_taxonomies(id)
        ON DELETE CASCADE,
    FOREIGN KEY (preparation_method_id)
        REFERENCES foods_preparation_methods(id)
        ON DELETE CASCADE
)

CREATE TABLE IF NOT EXISTS foods_food_variant_tag_relations (
    food_variant_id BLOB NOT NULL,
    food_tag_id BLOB NOT NULL,
    PRIMARY KEY (food_variant_id, food_tag_Id),
    FOREIGN KEY (food_variant_id)
        REFERENCES foods_food_variant_id(id)
        ON DELETE CASCADE,
    FOREIGN KEY (food_tag_id)
        REFERENCES foods_food_tags(id)
        ON DELETE CASADE
)

CREATE TABLE IF NOT EXISTS foods_food_variant_instance_relations (
    food_variant_id BLOB NOT NULL,
    food_instance_id BLOB NOT NULL,
    PRIMARY KEY (food_variant_id, food_tag_Id),
    FOREIGN KEY (food_variant_id)
        REFERENCES foods_food_variant_id(id)
        ON DELETE CASCADE,
    FOREIGN KEY (food_instance_id)
        REFERENCES foods_food_instances(id)
        ON DELETE CASADE
)
