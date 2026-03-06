use std::collections::BTreeSet;

use utils::database::DatabaseService;
use uuid::Uuid;

use crate::{nutrient_quantity::NutrientQuantity, nutrient_quantity_list::{self, NutrientQuantityList}};

pub struct NutrientQuantityListRecord {
    id: Vec<u8>,
}

impl NutrientQuantityListRecord {
    pub fn from_nutrient_quantity_list(nutrient_quantity_list: NutrientQuantityList) -> Self {
        let id = nutrient_quantity_list.get_id().as_bytes().to_vec();
        Self { id }
    }

    pub fn to_nutrient_quantity_list(&self) -> NutrientQuantityList {
        let mut nutrient_quantity_list = NutrientQuantityList::new();
        nutrient_quantity_list.set_id(Uuid::from_slice(&self.id).unwrap());
        return nutrient_quantity_list
    }

    pub async fn save_to_database(&self) -> Result<(), sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_quantity_list_table (id)
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
            "DELETE FROM nutrients_nutrient_quantity_list_table WHERE id = ?",
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

pub struct NutrientQuantityListItemRecord {
    nutrient_quantity_list_id: Vec<u8>,
    nutrient_quantity_id: Vec<u8>,
}

impl NutrientQuantityListItemRecord {
    // pub async fn from_nutrient_quantity_list(nutrient_quantity_list: NutrientQuantityList) -> Vec<Self> {
    //     let database_service = DatabaseService::new().await.unwrap();
    //
    //     let nutrient_quantity_list_id = NutrientQuantityListRecord::from_nutrient_quantity_list(nutrient_quantity_list).get_id();
    //     for nutrient_quantity in nutrient_quantity_list.get_nutrient_amounts().iter() {
    //         // Do a select using nutrient_quantity_list_id and nutrient_quantity.id and quantity with joins?
    //         // Create list of missing ones
    //         // Outside of for loop bulk save the missing ones
    //     }
    // }
    //
    // pub async fn to_btree_map_from_vec(items: Vec<&Self>) -> BTreeSet<NutrientQuantity> {
    //     // Do a bulk select for the items in the nutrient_quantity_table
    //     // Make a BTreeSet
    // }

    pub async fn load_from_sqlite(nutrient_quantity_list_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

        Ok(
            sqlx::query_as!(
                NutrientQuantityListItemRecord,
                r#"
                    SELECT
                        nutrient_quantity_list_id,
                        nutrient_quantity_id
                    FROM nutrients_nutrient_quantity_list_items
                    WHERE
                        nutrient_quantity_list_id = ?
                "#,
                nutrient_quantity_list_id
            )
            .fetch_all(&database_service.pool)
            .await?
        )
    }

    pub async fn save_to_database(&self) -> Result<(), sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_quantity_list_items (nutrient_quantity_list_id, nutrient_quantity_id)
                VALUES (?, ?)
                ON CONFLICT DO NOTHING
            "#,
            self.nutrient_quantity_list_id,
            self.nutrient_quantity_id,
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
                    INSERT INTO nutrients_nutrient_quantity_list_items (nutrient_quantity_list_id, nutrient_quantity_id)
                    VALUES (?, ?)
                    ON CONFLICT DO NOTHING
                "#,
                item.nutrient_quantity_list_id,
                item.nutrient_quantity_id,
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
                DELETE FROM nutrients_nutrient_quantity_list_items 
                WHERE
                    nutrient_quantity_list_id = ?
                    AND nutrient_quantity_id = ?
            "#,
            self.nutrient_quantity_list_id,
            self.nutrient_quantity_id,
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
                    DELETE FROM nutrients_nutrient_quantity_list_items
                    WHERE
                        nutrient_quantity_list_id = ?
                        AND nutrient_quantity_id = ?
                "#,
                item.nutrient_quantity_list_id,
                item.nutrient_quantity_id,
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit();

        Ok(())
    }
}
