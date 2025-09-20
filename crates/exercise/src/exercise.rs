use uuid::Uuid;
use chrono::Duration;
// use units::calories_per_second::CaloriesPerSecond;

pub struct ExerciseAmount {
    duration: Duration,
    exercise: Exercise,
}

pub struct Exercise {
    id: Uuid,
    name: String,
    description: String,
    // calories_per_second: CaloriesPerSecond,
}
