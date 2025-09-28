use chrono::Duration;
use units::power::Power;
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct ExerciseAmount {
    duration: Duration,
    exercise: Exercise,
}

#[derive(Clone, PartialEq)]
pub struct Exercise {
    id: Uuid,
    name: String,
    description: String,
    power: Power,
}
