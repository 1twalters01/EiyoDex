use identity::{entity::Entity, inner_id::InnerIdType, Id, InnerId};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::{exercise::Exercise, exercise_list::ExerciseList, records::exercise_record::ExerciseRecord};

#[derive(Debug, Clone, PartialEq)]
pub struct ExerciseListRecord {
    id: Vec<u8>,
    name: String,
    description: String,
}

impl ExerciseListRecord {
    pub fn from_values(id: Vec<u8>, name: String, description: String) -> Self {
        Self { id, name, description }
    }

    pub fn from_exercise_list_entity(exercise_list_entity: Entity<ExerciseList>) -> Self {
        let id = exercise_list_entity.get_id().to_bytes().to_vec();
        let exercise_list = exercise_list_entity.get_inner();
        let name = exercise_list.get_name();
        let description = exercise_list.get_description();
        Self { id, name, description }
    }

    pub fn to_nutrient_list(&self) -> ExerciseList {
        let mut exercise_list = ExerciseList::new();
        exercise_list.set_name(self.name.clone());
        exercise_list.set_description(self.description.clone());

        return exercise_list;
    }

    pub fn to_nutrient_list_entity(&self) -> Entity<ExerciseList> {
        let mut exercise_list = ExerciseList::new();
        exercise_list.set_name(self.name.clone());
        exercise_list.set_description(self.description.clone());

        let id = Id::from_inner(InnerId::Uuid(Uuid::from_slice(&self.id).unwrap()));
        Entity::new_with_id(id, exercise_list)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO exercises_exercise_list_table (id, name, description)
                VALUES (?, ?, ?)
                ON CONFLICT (id) DO NOTHING
            "#,
            self.id,
            self.name,
            self.description,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn save_or_update_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO exercises_exercise_list_table (id, name, description)
                VALUES (?, ?, ?)
                ON CONFLICT (id) DO UPDATE SET
                    name = excluded.name,
                    description = excluded.description
            "#,
            self.id,
            self.name,
            self.description,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_all_from_sqlite(pool: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
                Select id, "name", "description" FROM exercises_exercise_list_table
            "#,
        )
            .fetch_all(pool)
            .await?;

        let exercise_list_record_vec: Vec<ExerciseListRecord> = rows
        .into_iter()
        .map(|row| ExerciseListRecord {
            id: row.id,
            name: row.name,
            description: row.description,
        })
        .collect();
        
        Ok(exercise_list_record_vec)
    }

    pub async fn delete_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM exercises_exercise_list_table WHERE id = ? AND name = ?",
            self.id,
            self.name,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_all_items_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM exercises_exercise_list_items
                WHERE
                    exercise_list_id = ?
            "#,
            self.id,
        )
        .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn load_nutrients_from_database(&self, pool: &Pool<Sqlite>) -> Result<Vec<Exercise>, sqlx::Error> {
        let exercise_list_items_record = ExerciseListItemRecord::load_all_from_sqlite(&self.get_id(), pool).await?;
        
        let mut exercise_vec: Vec<Exercise> = Vec::new();
        for record in exercise_list_items_record {
            let exercise = record.to_exercise(&pool).await?;
            exercise_vec.push(exercise);
        }

        Ok(exercise_vec)
    }

    pub fn get_id(&self) -> Vec<u8> {
        self.id.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct ExerciseListItemRecord {
    exercise_list_id: Vec<u8>,
    exercise_id: Vec<u8>,
}

impl ExerciseListItemRecord {
    pub fn from_value(exercise_list_id: Vec<u8>, exercise_id: Vec<u8>) -> Self {
        Self { exercise_list_id, exercise_id }
    }

    pub async fn from_exercise_list_entity(exercise_list_entity: Entity<ExerciseList>, pool: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
        let exercise_list_id = exercise_list_entity.get_id().to_bytes().to_vec();
        let exercise_list = exercise_list_entity.get_inner();
        let mut exercise_list_item_vec: Vec<Self> = Vec::new();

        for exercise in exercise_list.get_exercises() {
            let exercise_id = ExerciseRecord::load_from_database_using_name(exercise.borrow().get_name(), pool).await.unwrap().get_exercise_id();
            let item = Self {
                exercise_list_id: exercise_list_id.clone(),
                exercise_id,
            };
            exercise_list_item_vec.push(item);
        }

        return Ok(exercise_list_item_vec)
    }

    pub async fn to_exercise(&self, pool: &Pool<Sqlite>) -> Result<Exercise, sqlx::Error> {
        let exercise_record = ExerciseRecord::load_from_database_using_id(Id::from_inner(InnerId::from_slice(InnerIdType::Uuid, &self.exercise_id).unwrap()), pool);
        exercise_record.await?.to_exercise(pool).await
    }

    pub async fn load_all_from_sqlite(exercise_list_id: &Vec<u8>, pool: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(
            ExerciseListItemRecord,
            r#"
                SELECT
                    exercise_list_id,
                    exercise_id
                FROM exercises_exercise_list_items
                WHERE
                    exercise_list_id = ?
            "#,
            exercise_list_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO exercises_exercise_list_items (exercise_list_id, exercise_id)
                VALUES (?, ?)
                ON CONFLICT DO NOTHING
            "#,
            self.exercise_list_id,
            self.exercise_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn save_vec_to_database(items: Vec<&Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for item in items {
            sqlx::query!(
                r#"
                    INSERT INTO exercises_exercise_list_items (exercise_list_id, exercise_id)
                    VALUES (?, ?)
                    ON CONFLICT(exercise_list_id, exercise_id) DO NOTHING 
                "#,
                item.exercise_list_id,
                item.exercise_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_item_from_sqlite(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM exercises_exercise_list_items 
                WHERE
                    exercise_list_id = ?
                    AND exercise_id = ?
            "#,
            self.exercise_list_id,
            self.exercise_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_item_vec_from_sqlite(items: Vec<&Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for item in items {
            sqlx::query!(
                r#"
                    DELETE FROM exercises_exercise_list_items
                    WHERE
                        exercise_list_id = ?
                        AND exercise_id = ?
                "#,
                item.exercise_list_id,
                item.exercise_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
