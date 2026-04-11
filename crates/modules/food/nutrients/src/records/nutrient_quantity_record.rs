use identity::{inner_id::InnerIdType, Id};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::{
    nutrient_quantity::NutrientQuantity,
    records::{nutrient_record::NutrientRecord, nutrient_unit_record::NutrientUnitRecord},
};

#[derive(Debug, PartialEq, Clone)]
pub struct NutrientQuantityRecord {
    pub id: Vec<u8>,
    pub quantity: f64,
    pub nutrient_id: Vec<u8>,
    pub output_unit_id: i64,
}

impl NutrientQuantityRecord {
    pub fn from_values(id: Vec<u8>, quantity: f64, nutrient_id: Vec<u8>, output_unit_id: i64) -> Self {
        Self { id, quantity, nutrient_id, output_unit_id }
    }

    pub async fn from_nutrient_quantity(nutrient_quantity: NutrientQuantity, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        let id = Id::<NutrientQuantity>::new(InnerIdType::Uuid).to_bytes().to_vec();
        let quantity = nutrient_quantity.get_value();
        let nutrient_id = NutrientRecord::from_nutrient(nutrient_quantity.get_nutrient().borrow().clone(), pool).await?.nutrient_id;

        let output_unit = nutrient_quantity.get_output_unit();
        let output_unit_id = NutrientUnitRecord::from_nutrient_unit(output_unit, pool).await.get_database_id(pool).await?;

        Ok(Self { id, quantity, nutrient_id, output_unit_id })
    }


    pub async fn to_nutrient_quantity(&self, pool: &Pool<Sqlite>) -> Result<NutrientQuantity, sqlx::Error> {
        let quantity = self.quantity;
        let nutrient =
            NutrientRecord::load_from_database_using_id(Id::from_slice(InnerIdType::Uuid, &self.nutrient_id).unwrap(), pool)
                .await?
                .to_nutrient(pool)
                .await?;
        let output_unit = NutrientUnitRecord::load_from_database(self.output_unit_id, pool)
            .await?
            .to_nutrient_unit(pool)
            .await;

        let nutrient_quantity = NutrientQuantity::new(quantity, nutrient, output_unit).unwrap();
        Ok(nutrient_quantity)
    }

    pub async fn load_from_database(id: Vec<u8>, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
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
