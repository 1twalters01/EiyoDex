CREATE TABLE IF NOT EXISTS foods_food_instances (
    id BLOB PRIMARY KEY,
    data_source_instance_id BLOB NOT NULL,
    FOREIGN KEY (data_source_instance_id)
        REFERENCES foods_data_source_instances(id)
        ON DELETE CASCADE,
)

