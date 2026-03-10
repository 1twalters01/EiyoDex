use sqlx::{Pool, Sqlite};
use utils::database::DatabaseService;
use uuid::Uuid;

use crate::{
    nutrient_quantity::NutrientQuantity,
    records::{nutrient_record::NutrientRecord, nutrient_unit_record::NutrientUnitRecord},
};

pub struct NutrientQuantityRecord {
    pub id: Vec<u8>,
    pub quantity: f64,
    pub nutrient_id: Vec<u8>,
    pub output_unit_id: i64,
}

impl NutrientQuantityRecord {
    pub async fn from_nutrient_quantity(nutrient_quantity: NutrientQuantity, pool: &Pool<Sqlite>) -> Self {
        let id: Vec<u8> = nutrient_quantity.get_id().as_bytes().to_vec();
        let quantity = nutrient_quantity.get_value();
        let nutrient_id = nutrient_quantity
            .get_nutrient()
            .borrow()
            .get_id()
            .as_bytes()
            .to_vec();
        let output_unit_id =
            NutrientUnitRecord::from_nutrient_unit(nutrient_quantity.get_output_unit(), pool)
                .await
                .get_unit_type_id()
                .expect("Invalid unit");

        Self {
            id,
            quantity,
            nutrient_id,
            output_unit_id,
        }
    }

    pub async fn to_nutrient_quantity(&self, pool: &Pool<Sqlite>) -> NutrientQuantity {
        let id = Uuid::from_slice(&self.id).unwrap();
        let quantity = self.quantity;
        let nutrient =
            NutrientRecord::load_from_database(Uuid::from_slice(&self.nutrient_id).unwrap(), pool)
                .await
                .unwrap()
                .to_nutrient(pool)
                .await;
        let output_unit = NutrientUnitRecord::load_from_database(self.output_unit_id, pool)
            .await
            .unwrap()
            .to_nutrient_unit(pool)
            .await;

        let mut nutrient_quantity = NutrientQuantity::new(quantity, nutrient, output_unit).unwrap();
        nutrient_quantity.set_id(id);
        return nutrient_quantity;
    }

    pub async fn load_from_database(id: Uuid, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        Ok(sqlx::query_as!(
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
            id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_quantity_table (id, quantity, nutrient_id, output_unit_id)
                VALUES (?, ?, ?, ?)
                ON CONFLICT (id)
                DO UPDATE SET
                    id = excluded.id,
                    quantity = excluded.quantity,
                    nutrient_id = excluded.nutrient_id,
                    output_unit_id = excluded.output_unit_id
            "#,
            self.id,
            self.quantity,
            self.nutrient_id,
            self.output_unit_id,
        )
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_nutrient_quantity(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM nutrients_nutrient_quantity_table WHERE id = ?",
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
