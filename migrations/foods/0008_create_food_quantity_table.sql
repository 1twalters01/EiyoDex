CREATE TABLE IF NOT EXISTS foods_food_quantity (
    id BLOB PRIMARY KEY NOT NULL,
    value REAL NOT NULL,
    food_variant_id BLOB NOT NULL,
    data_source_provider_id BLOB NOT NULL,
    data_source_version_id BLOB NOT NULL,
    -- Timestamp with unix epoch
    created_at INTEGER NOT NULL,
    last_modified INTEGER NOT NULL,
    FOREIGN KEY (food_variant_id)
        REFERENCES foods_food_variants(id)
        ON DELETE CASCADE,
    FOREIGN KEY (data_source_provider_id)
        REFERENCES foods_data_source_providers(id)
        ON DELETE CASCADE,
    FOREIGN KEY (data_source_version_id)
        REFERENCES foods_data_source_versions(id)
        ON DELETE CASCADE,
);

