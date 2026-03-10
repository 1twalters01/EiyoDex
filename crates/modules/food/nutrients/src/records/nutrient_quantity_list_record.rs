use std::collections::BTreeSet;

use sqlx::{Pool, Sqlite};
use utils::database::DatabaseService;
use uuid::Uuid;

use crate::{
    nutrient_quantity::NutrientQuantity, nutrient_quantity_list::NutrientQuantityList,
    records::nutrient_quantity_record::NutrientQuantityRecord,
};

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
        return nutrient_quantity_list;
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_quantity_list_table (id)
                VALUES (?)
                ON CONFLICT DO NOTHING
            "#,
            self.id,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM nutrients_nutrient_quantity_list_table WHERE id = ?",
            self.id
        )
        .execute(pool)
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
    pub async fn from_nutrient_quantity_list(
        nutrient_quantity_list: NutrientQuantityList,
    ) -> Vec<Self> {
        let mut record_vec: Vec<NutrientQuantityListItemRecord> = Vec::new();
        let nutrient_quantity_list_id =
            NutrientQuantityListRecord::from_nutrient_quantity_list(nutrient_quantity_list.clone())
                .get_id();
        for nutrient_quantity in nutrient_quantity_list.get_nutrient_quantities().iter() {
            let nutrient_quantity_id = nutrient_quantity.get_id().as_bytes().to_vec();
            let nutrient_quantity_list_item = NutrientQuantityListItemRecord {
                nutrient_quantity_list_id: nutrient_quantity_list_id.clone(),
                nutrient_quantity_id,
            };
            record_vec.push(nutrient_quantity_list_item);
        }

        return record_vec;
    }

    pub async fn to_btree_map_from_vec(
        items: Vec<&Self>, pool: &Pool<Sqlite>,
    ) -> Result<BTreeSet<NutrientQuantity>, sqlx::Error> {
        if let Some(first) = items.first() {
            if !items
                .iter()
                .all(|item| item.nutrient_quantity_list_id == first.nutrient_quantity_list_id)
            {
                panic!("List has elements of a different list")
            };
        }

        let mut tx = pool.begin().await?;

        let mut quantity_tree: BTreeSet<NutrientQuantity> = BTreeSet::new();
        for item in items {
            let nutrient_quantity_record = sqlx::query_as!(
                NutrientQuantityRecord,
                r#"
                    SELECT
                        id,
                        quantity,
                        nutrient_id,
                        output_unit_id
                    FROM nutrients_nutrient_quantity_table
                    WHERE
                        id = ?
                "#,
                item.nutrient_quantity_id
            )
            .fetch_one(&mut *tx)
            .await?;

            let nutrient_quantity = nutrient_quantity_record.to_nutrient_quantity(pool).await;
            quantity_tree.insert(nutrient_quantity);
        }

        return Ok(quantity_tree);
    }

    pub async fn load_from_sqlite(
        nutrient_quantity_list_id: Uuid, pool: &Pool<Sqlite>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(
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
        .fetch_all(pool)
        .await?)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_quantity_list_items (nutrient_quantity_list_id, nutrient_quantity_id)
                VALUES (?, ?)
                ON CONFLICT DO NOTHING
            "#,
            self.nutrient_quantity_list_id,
            self.nutrient_quantity_id,
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

    pub async fn delete_conversion(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
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
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_conversion_vec(items: Vec<&Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

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
        tx.commit().await?;

        Ok(())
    }
}
