use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::{
    nutrient_list::NutrientList,
    records::nutrient_record::NutrientRecord,
};

#[derive(Debug, PartialEq)]
pub struct NutrientListRecord {
    id: Vec<u8>,
}

impl NutrientListRecord {
    pub fn from_value(id: Vec<u8>) -> Self {
        Self { id }
    }

    pub fn from_nutrient_list(nutrient_list: NutrientList) -> Self {
        let id = nutrient_list.get_id().as_bytes().to_vec();
        Self { id }
    }

    pub fn to_nutrient_quantity_list(&self) -> NutrientList {
        let mut nutrient_list = NutrientList::new();
        nutrient_list.set_id(Uuid::from_slice(&self.id).unwrap());

        return nutrient_list;
    }

    pub fn get_id(&self) -> Vec<u8> {
        self.id.clone()
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_list_table (id)
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
            "DELETE FROM nutrients_nutrient_list_table WHERE id = ?",
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
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
            // let nutrient_id = NutrientRecord::from_nutrient(nutrient.borrow().clone(), pool).await.unwrap().nutrient_id;
            let item = Self {
                nutrient_list_id: nutrient_list_id.clone(),
                nutrient_id,
            };
            nutrient_list_item_vec.push(item);
        }

        return Ok(nutrient_list_item_vec)
    }

//     pub async fn to_nutrient_vec_from_from_vec(
//         items: Vec<&Self>,
//         pool: &Pool<Sqlite>,
//     ) -> Result<Vec<(Nutrient, NutrientLinkRecordUuid)>, sqlx::Error> {
//         let mut tx = pool.begin().await?;
//
//         let mut nutrient_map: Vec<(Nutrient, NutrientLinkRecordUuid)> = Vec::new();
//         for item in items {
//             let nutrient_record = sqlx::query_as!(
//                 NutrientRecord,
//                 r#"
//                     SELECT
//                         id as nutrient_id,
//                         name,
//                         description,
//                         main_unit_id,
//                         quantity_type_id,
//                         essentiality_type_id,
//                         chemical_id
//                     FROM nutrients_nutrient_table
//                     WHERE
//                         id = ?
//                 "#,
//                 item.nutrient_id
//             )
//             .fetch_one(&mut *tx)
//             .await?;
//
//             let conversion_vec = sqlx::query_as!(
//                 NutrientConversionRecord,
//                 r#"
//                     SELECT
//                         nutrient_id,
//                         unit_id,
//                         factor
//                     FROM nutrients_unit_conversions
//                     WHERE
//                         nutrient_id = ?
//                 "#,
//                 item.nutrient_id
//             )
//             .fetch_all(&mut *tx)
//             .await?;
//             let unit_conversions =
//                 NutrientConversionRecord::to_btree_map_from_vec(conversion_vec, pool).await;
//
//             let parent_rows = sqlx::query!(
//                 r#"
//                 SELECT
//                     parent_id
//                 FROM nutrients_nutrient_relationships
//                 WHERE
//                     child_id = ?
//             "#,
//                 item.nutrient_id
//             )
//             .fetch_all(&mut *tx)
//             .await?;
//             let parent_id_vec: Vec<Uuid> = parent_rows
//                 .iter()
//                 .map(|row| Uuid::from_slice(&row.parent_id).unwrap())
//                 .collect();
//
//             let child_rows = sqlx::query!(
//                 r#"
//                 SELECT
//                     child_id
//                 FROM nutrients_nutrient_relationships
//                 WHERE
//                     parent_id = ?
//             "#,
//                 item.nutrient_id
//             )
//             .fetch_all(&mut *tx)
//             .await?;
//             let child_id_vec: Vec<Uuid> = child_rows
//                 .iter()
//                 .map(|row| Uuid::from_slice(&row.child_id).unwrap())
//                 .collect();
//
//             let nutrient_id = Uuid::from_slice(&item.nutrient_id).unwrap();
//             let nutrient_link_record_uuid = NutrientLinkRecordUuid {
//                 nutrient_id,
//                 parent_id_vec,
//                 child_id_vec,
//             };
//
//             let mut nutrient = nutrient_record.to_nutrient(pool).await;
//             nutrient.set_unit_conversions(unit_conversions);
//             nutrient_map.push((nutrient, nutrient_link_record_uuid));
//         }
//
//         let _ = tx.commit();
//
//         Ok(nutrient_map)
//     }
//
    pub async fn load_all_from_sqlite(nutrient_list_id: Uuid, pool: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
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

//     pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
//         sqlx::query!(
//             r#"
//                 INSERT INTO nutrients_nutrient_list_items (nutrient_list_id, nutrient_id)
//                 VALUES (?, ?)
//                 ON CONFLICT DO NOTHING
//             "#,
//             self.nutrient_list_id,
//             self.nutrient_id,
//         )
//         .execute(pool)
//         .await?;
//         Ok(())
//     }
//
//     pub async fn save_vec_to_database(items: Vec<&Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
//         let mut tx = pool.begin().await?;
//
//         for item in items {
//             sqlx::query!(
//                 r#"
//                     INSERT INTO nutrients_nutrient_list_items (nutrient_list_id, nutrient_id)
//                     VALUES (?, ?)
//                     ON CONFLICT DO NOTHING
//                 "#,
//                 item.nutrient_list_id,
//                 item.nutrient_id,
//             )
//             .execute(&mut *tx)
//             .await?;
//         }
//         Ok(())
//     }
//
//     pub async fn delete_conversion(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
//         sqlx::query!(
//             r#"
//                 DELETE FROM nutrients_nutrient_list_items 
//                 WHERE
//                     nutrient_list_id = ?
//                     AND nutrient_id = ?
//             "#,
//             self.nutrient_list_id,
//             self.nutrient_id,
//         )
//         .execute(pool)
//         .await?;
//
//         Ok(())
//     }
//
//     pub async fn delete_conversion_vec(items: Vec<&Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
//         let mut tx = pool.begin().await?;
//
//         for item in items {
//             sqlx::query!(
//                 r#"
//                     DELETE FROM nutrients_nutrient_list_items
//                     WHERE
//                         nutrient_list_id = ?
//                         AND nutrient_id = ?
//                 "#,
//                 item.nutrient_list_id,
//                 item.nutrient_id,
//             )
//             .execute(&mut *tx)
//             .await?;
//         }
//
//         let _ = tx.commit();
//
//         Ok(())
//     }
}
