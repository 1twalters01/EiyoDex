use std::collections::BTreeSet;

use utils::database::DatabaseService;
use uuid::Uuid;

use crate::{nutrient::Nutrient, nutrient_list::NutrientList};

pub struct NutrientListRecord {
    id: Vec<u8>,
}

impl NutrientListRecord {
    pub fn from_nutrient_list(nutrient_list: NutrientList) -> Self {
        let id = nutrient_list.get_id().as_bytes().to_vec();
        Self { id }
    }

    pub fn to_nutrient_quantity_list(&self) -> NutrientList {
        let mut nutrient_list = NutrientList::new();
        nutrient_list.set_id(Uuid::from_slice(&self.id).unwrap());
        return nutrient_list
    }

    pub async fn save_to_database(&self) -> Result<(), sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_list_table (id)
                VALUES (?)
                ON CONFLICT DO NOTHING
            "#,
            self.id,
        )
            .execute(&database_service.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_from_database(&self) -> Result<(), sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

        sqlx::query!(
            "DELETE FROM nutrients_nutrient_list_table WHERE id = ?",
            self.id
        )
        .execute(&database_service.pool)
        .await?;

        Ok(())
    }

    pub fn get_id(&self) -> Vec<u8> {
        self.id.clone()
    }
}

pub struct NutrientListItemRecord {
    nutrient_list_id: Vec<u8>,
    nutrient_id: Vec<u8>,
}


impl NutrientListItemRecord {
    // pub async fn from_nutrient_list(nutrient_list: NutrientList) -> Vec<Self> {
    // }
    // 
    // pub async fn to_btree_map_from_vec(items: Vec<&Self>) -> BTreeSet<Nutrient> {
    // }

    pub async fn load_from_sqlite(nutrient_list_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

        Ok(
            sqlx::query_as!(
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
            .fetch_all(&database_service.pool)
            .await?
        )
    }

    pub async fn save_to_database(&self) -> Result<(), sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_list_items (nutrient_list_id, nutrient_id)
                VALUES (?, ?)
                ON CONFLICT DO NOTHING
            "#,
            self.nutrient_list_id,
            self.nutrient_id,
        )
            .execute(&database_service.pool)
            .await?;
        Ok(())
    }

    pub async fn save_vec_to_database(items: Vec<&Self>) -> Result<(), sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();
        let mut tx = database_service.pool.begin().await?;

        for item in items {
            sqlx::query!(
                r#"
                    INSERT INTO nutrients_nutrient_list_items (nutrient_list_id, nutrient_id)
                    VALUES (?, ?)
                    ON CONFLICT DO NOTHING
                "#,
                item.nutrient_list_id,
                item.nutrient_id,
            )
                .execute(&mut *tx)
                .await?;
        }
        Ok(())
    }

    pub async fn delete_conversion(&self) -> Result<(), sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

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
        .execute(&database_service.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_conversion_vec(items: Vec<&Self>) -> Result<(), sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();
        let mut tx = database_service.pool.begin().await?;

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

        Ok(())
    }
}
