CREATE TABLE IF NOT EXISTS journal_exercise_table (
    id INTEGER PRIMary key,
    food_quantity_id INTEGER,
    exercise_quantity_id INTEGER,
    FOREIGN KEY (food_quantity_id)
        REFERENCES foods_food_quantities(id)
        ON DELETE CASCADE,
    FOREIGN KEY (exercise_quantity_id)
        REFERENCES exercises_exercise_quantities(id)
        ON DELETE CASCADE,
);
