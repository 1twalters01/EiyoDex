use sqlx::{Pool, Sqlite};
use crate::exercise::Exercise

#[derive(Debug, PartialEq)]
pub struct ExerciseRecord {
    exercise_id: i64,
    name: String,
    description: String,
    power_id: i64,
}

impl ExerciseRecord {
    pub fn from_values(name: String, description: String, power_id: i64) -> Self {
        Self { name, description, power_id }
    }

    pub async fn from_exercise(exercise: Exercise) -> Self {
      let name = exercise.get_name();
      let description = exercise.get_description();
      let power_id =  Some(exercise.get_power().get_id();
      Self { name, description, power_id }
    }

    pub async fn to_exercise(&self, pool: &Pool<Sqlite>) -> Exercise {}

    pub async fn to_exercise_entity(&self, pool: &Pool<Sqlite>) -> Entity<Exercise> {}

    pub async fn select_exercise_from_exercise_list(&self, exercise_list: ExerciseList, pool: &Pool<Sqlite>) -> Option<Rc<RefCell<Exercise>>> {}

    pub async fn load_from_database_using_name(name: String, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error>{}

    pub async fn load_from_database_using_id(exercise_id: Id<Exercise>, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {}

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<Vec<u8>, sqlx::Error> {}

    pub async fn delete_exercise_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {}
}
