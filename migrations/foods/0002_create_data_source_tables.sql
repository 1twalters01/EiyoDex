CREATE TABLE IF NOT EXISTS foods_data_source_providers (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS foods_data_source_versions (
    id BLOB PRIMARY KEY NOT NULL,
    version TEXT NOT NULL,
    description TEXT NOT NULL
)

CREATE TABLE IF NOT EXISTS foods_data_source_instances (
    data_source_provider_id BLOB NOT NULL,
    data_source_version_id BLOB NOT NULL,
    description TEXT NOT NULL,
    nutrient_quantity_list_id BLOB NOT NULL,
    PRIMARY KEY (data_source_provider_id, data_source_version_id)
    FOREIGN KEY (data_source_provider_id)
        REFERENCES foods_data_source_providers(id)
        ON DELETE CASCADE,
    FOREIGN KEY (foods_data_source_version_id)
        REFERENCES foods_data_source_versions(id)
        ON DELETE CASCADE,
    FOREIGN KEY (nutrient_quantity_list_id)
        REFERENCES nutrients_nutrient_quantity_list_table(id)
        ON DELETE CASCADE
)
