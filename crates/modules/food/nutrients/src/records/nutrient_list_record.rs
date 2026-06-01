use identity::{inner_id::InnerIdType, Id, InnerId};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::{
    nutrient::Nutrient, nutrient_list::NutrientList, records::nutrient_record::NutrientRecord
};

#[derive(Debug, Clone, PartialEq)]
pub struct NutrientListRecord {
    id: Vec<u8>,
    name: String,
    description: String,
}

impl NutrientListRecord {
    pub fn from_value(id: Vec<u8>, name: String, description: String) -> Self {
        Self { id, name, description }
    }

    pub fn from_nutrient_list(nutrient_list: NutrientList) -> Self {
        let id = nutrient_list.get_id().as_bytes().to_vec();
        let name = nutrient_list.get_name();
        let description = nutrient_list.get_description();
        Self { id, name, description }
    }

    pub fn to_nutrient_list(&self) -> NutrientList {
        let mut nutrient_list = NutrientList::new();
        nutrient_list.set_id(Uuid::from_slice(&self.id).unwrap());
        nutrient_list.set_name(self.name.clone());
        nutrient_list.set_description(self.description.clone());

        return nutrient_list;
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_list_table (id, name, description)
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
                INSERT INTO nutrients_nutrient_list_table (id, name, description)
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
                Select id, "name", "description" FROM nutrients_nutrient_list_table
            "#,
        )
            .fetch_all(pool)
            .await?;

        let nutrient_list_record_vec: Vec<NutrientListRecord> = rows
        .into_iter()
        .map(|row| NutrientListRecord {
            id: row.id,
            name: row.name,
            description: row.description,
        })
        .collect();
        
        Ok(nutrient_list_record_vec)
    }

    pub async fn delete_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM nutrients_nutrient_list_table WHERE id = ? AND name = ?",
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
                DELETE FROM nutrients_nutrient_list_items
                WHERE
                    nutrient_list_id = ?
            "#,
            self.id,
        )
        .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn load_nutrients_from_database(&self, pool: &Pool<Sqlite>) -> Result<Vec<Nutrient>, sqlx::Error> {
        let nutrient_list_items_record = NutrientListItemRecord::load_all_from_sqlite(&self.get_id(), pool).await?;
        
        let mut nutrient_vec: Vec<Nutrient> = Vec::new();
        for record in nutrient_list_items_record {
            let nutrient = record.to_nutrient(&pool).await?;
            nutrient_vec.push(nutrient);
        }

        Ok(nutrient_vec)
    }

    pub fn get_id(&self) -> Vec<u8> {
        self.id.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct NutrientListItemRecord {
    nutrient_list_id: Vec<u8>,
    nutrient_id: Vec<u8>,
}

impl NutrientListItemRecord {
    pub fn from_value(nutrient_list_id: Vec<u8>, nutrient_id: Vec<u8>) -> Self {
        Self { nutrient_list_id, nutrient_id }
    }

    pub async fn from_nutrient_list(nutrient_list: NutrientList, pool: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
        let nutrient_list_id = nutrient_list.get_id().as_bytes().to_vec();
        let mut nutrient_list_item_vec: Vec<Self> = Vec::new();

        for nutrient in nutrient_list.get_nutrients() {
            let nutrient_id = NutrientRecord::load_from_database_using_name(nutrient.borrow().get_name(), pool).await.unwrap().nutrient_id;
            let item = Self {
                nutrient_list_id: nutrient_list_id.clone(),
                nutrient_id,
            };
            nutrient_list_item_vec.push(item);
        }

        return Ok(nutrient_list_item_vec)
    }

    pub async fn to_nutrient(&self, pool: &Pool<Sqlite>) -> Result<Nutrient, sqlx::Error> {
        let nutrient_record = NutrientRecord::load_from_database_using_id(Id::from_inner(InnerId::from_slice(InnerIdType::Uuid, &self.nutrient_id).unwrap()), pool);
        nutrient_record.await?.to_nutrient(pool).await
    }

    pub async fn load_all_from_sqlite(nutrient_list_id: &Vec<u8>, pool: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(
            NutrientListItemRecord,
            r#"
                SELECT
                    nutrient_list_id,
                    nutrient_id
                FROM nutrients_nutrient_list_items
                WHERE
                    nutrient_list_id = ?
            "#,
            nutrient_list_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_list_items (nutrient_list_id, nutrient_id)
                VALUES (?, ?)
                ON CONFLICT DO NOTHING
            "#,
            self.nutrient_list_id,
            self.nutrient_id,
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
                    INSERT INTO nutrients_nutrient_list_items (nutrient_list_id, nutrient_id)
                    VALUES (?, ?)
                    ON CONFLICT(nutrient_list_id, nutrient_id) DO NOTHING 
                "#,
                item.nutrient_list_id,
                item.nutrient_id,
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
                DELETE FROM nutrients_nutrient_list_items 
                WHERE
                    nutrient_list_id = ?
                    AND nutrient_id = ?
            "#,
            self.nutrient_list_id,
            self.nutrient_id,
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
                    DELETE FROM nutrients_nutrient_list_items
                    WHERE
                        nutrient_list_id = ?
                        AND nutrient_id = ?
                "#,
                item.nutrient_list_id,
                item.nutrient_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
