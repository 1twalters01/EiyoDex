use std::{cell::RefCell, rc::Rc};

use identity::{
    entity::{Entity, GetFromDatabaseUsingId},
    inner_id::InnerIdType,
    Id
};
use sqlx::{Pool, Sqlite};
use units::power::quantity::PowerQuantity;
use crate::{exercise::Exercise, exercise_list::ExerciseList};

#[derive(Debug, PartialEq)]
pub struct ExerciseRecord {
    exercise_id: Vec<u8>,
    name: String,
    description: String,
    power_quantity_id: Vec<u8>,
}

impl ExerciseRecord {
    pub fn from_values(exercise_id: Vec<u8>, name: String, description: String, power_quantity_id: Vec<u8>) -> Self {
        Self { exercise_id, name, description, power_quantity_id }
    }

    pub async fn from_exercise(exercise: Exercise, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        // get exercise id
        let exercise_record_option = Self::load_from_database_using_name(exercise.get_name(), &pool).await;
        let exercise_id = match exercise_record_option {
            Ok(record) => record.exercise_id,
            Err(sqlx::Error::RowNotFound) => Id::<Exercise>::new(InnerIdType::Uuid).to_bytes().to_vec(),
            Err(err) => return Err(err),
        };

        // get name and description
        let name = exercise.get_name();
        let description = exercise.get_description();

        // get power quantity id
        let power_quantity_entity = exercise.get_power_quantity_entity();
        let power_quantity_id = power_quantity_entity.save_to_database(&pool).await?;

        Ok(Self { exercise_id, name, description, power_quantity_id })
    }

    pub async fn from_exercise_entity(exercise_entity: Entity<Exercise>, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        let exercise_id = exercise_entity.id.to_bytes().to_vec();
        let exercise = exercise_entity.get_inner();
        let name = exercise.get_name();
        let description = exercise.get_description();
        let power_quantity_entity = exercise.get_power_quantity_entity();
        let power_quantity_id = power_quantity_entity.save_to_database(&pool).await?;

        Ok(Self { exercise_id, name, description, power_quantity_id })
    }

    pub async fn to_exercise(&self, pool: &Pool<Sqlite>) -> Result<Exercise, sqlx::Error> {
        let name = self.name.clone();
        let description = self.description.clone();
        let power_quantity_id = Id::from_slice(InnerIdType::Uuid, &self.power_quantity_id).unwrap();
        let power_quantity_entity = PowerQuantity::get_from_database_using_id(power_quantity_id, pool).await?;
        let exercise = Exercise::from_values(name, description, power_quantity_entity);
        return Ok(exercise)
    }

    pub async fn to_exercise_entity(&self, pool: &Pool<Sqlite>) -> Result<Entity<Exercise>, sqlx::Error> {
        let exercise_id = Id::<Exercise>::from_slice(InnerIdType::Uuid, &self.exercise_id).unwrap();
        let name = self.name.clone();
        let description = self.description.clone();
        let power_quantity_id = Id::from_slice(InnerIdType::Uuid, &self.power_quantity_id).unwrap();
        let power_quantity_entity = PowerQuantity::get_from_database_using_id(power_quantity_id, pool).await?;
        let exercise = Exercise::from_values(name, description, power_quantity_entity);
        Ok(Entity::new_with_id(exercise_id, exercise))
    }

    pub async fn select_exercise_from_exercise_list(&self, exercise_list: ExerciseList) -> Option<Rc<RefCell<Exercise>>> {
        let exercises = exercise_list.get_exercises();
        exercises.iter().find(|exercise| exercise.borrow().get_name() == self.name).cloned()
    }

    pub async fn load_from_database_using_name(name: String, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        Ok(sqlx::query_as!(
            Self,
            r#"
                    SELECT
                        id as exercise_id,
                        name,
                        description,
                        power_quantity_id
                    FROM exercises_exercise_table
                    WHERE
                        name = ?
                "#,
            name
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn load_from_database_using_id(exercise_id: Id<Exercise>, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        let id = exercise_id.get_inner().to_bytes().to_vec();

        Ok(sqlx::query_as!(
            Self,
            r#"
                    SELECT
                        id as exercise_id,
                        name,
                        description,
                        power_quantity_id
                    FROM exercises_exercise_table
                    WHERE
                        id = ?
                "#,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<Vec<u8>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
                INSERT INTO exercises_exercise_table (id, name, description, power_quantity_id)
                VALUES (?, ?, ?, ?)
                ON CONFLICT (name)
                DO UPDATE SET
                    description = excluded.description,
                    power_quantity_id = excluded.power_quantity_id
                RETURNING id
            "#,
            self.exercise_id,
            self.name,
            self.description,
            self.power_quantity_id,
        )
            .fetch_one(pool)
            .await?;
        Ok(row.id)
 }

    pub async fn delete_exercise_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM exercises_exercise_table WHERE id = ?",
            self.exercise_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub fn get_exercise_id(&self) -> Vec<u8> {
        self.exercise_id.clone()
    }
}
