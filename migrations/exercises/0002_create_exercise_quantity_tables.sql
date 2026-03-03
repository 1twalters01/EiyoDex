CREATE TABLE IF NOT EXISTS exercises_nutrient_quantity_table (
    id INTEGER PRIMARY KEY,
    duration_quantity_id INTEGER NOT NULL,
    exercise_id INTEGER NOT NULL,
    FOREIGN KEY (duration_type_id)
        REFERENCES units_duration_quantities(id)
        ON DELETE CASCADE,
    FOREIGN KEY (duration_type_id)
        REFERENCES exercises_exercise_table(id)
        ON DELETE CASCADE
);
