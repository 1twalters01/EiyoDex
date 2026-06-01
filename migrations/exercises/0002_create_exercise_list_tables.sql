CREATE TABLE IF NOT EXISTS exercises_exercise_list_table (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS exercises_exercise_list_items (
    exercise_list_id BLOB NOT NULL,
    exercise_id BLOB NOT NULL,
    PRIMARY KEY (exercise_list_id, exercise_id),
    FOREIGN KEY (exercise_list_id)
        REFERENCES exercises_exercise_list_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (exercise_id)
        REFERENCES exercises_exercise_table(id)
        ON DELETE CASCADE
);

