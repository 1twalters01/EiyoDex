use chrono::Duration;
use units::power::Power;
use uuid::Uuid;

pub struct ExerciseAmount {
    duration: Duration,
    exercise: Exercise,
}

pub struct Exercise {
    id: Uuid,
    name: String,
    description: String,
    power: Power,
}
