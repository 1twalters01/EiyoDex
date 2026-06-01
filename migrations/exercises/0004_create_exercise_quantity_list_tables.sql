CREATE TABLE IF NOT EXISTS exercises_exercise_quantity_list_table (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS exercises_exercise_quantity_list_items (
    exercise_quantity_list_id BLOB NOT NULL,
    exercise_quantity_id BLOB NOT NULL,
    PRIMARY KEY (exercise_quantity_list_id, exercise_quantity_id),
    FOREIGN KEY (exercise_quantity_list_id)
        REFERENCES exercises_exercise_quantity_list_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (exercise_quantity_id)
        REFERENCES exercises_exercise_quantity_table(id)
        ON DELETE CASCADE
);

