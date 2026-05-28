CREATE TABLE IF NOT EXISTS exercises_exercise_table (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    power_quantity_id BLOB NOT NULL,
    FOREIGN KEY (power_quantity_id)
        REFERENCES units_power_quantities(id)
        ON DELETE RESTRICT,
    CHECK (LENGTH(id) = 16)
)
