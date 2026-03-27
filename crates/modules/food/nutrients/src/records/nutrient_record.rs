use std::collections::BTreeMap;

use identity::{inner_id::InnerIdType, Id};
use sqlx::{Pool, Sqlite};
use utils::dsa::node::GraphNode;
use uuid::Uuid;

use crate::{
    entity::Entity, nutrient::Nutrient, nutrient_units::NutrientUnit, records::{nutrient_type_record::NutrientTypeRecord, nutrient_unit_record::NutrientUnitRecord}
};

#[derive(Debug, PartialEq)]
pub struct NutrientRecord {
    pub nutrient_id: Vec<u8>,
    pub name: String,
    pub description: String,
    pub main_unit_id: i64,
    pub essentiality_type_id: i64,
    pub quantity_type_id: i64,
    pub chemical_id: i64,
}

impl NutrientRecord {
    pub fn from_values(
        nutrient_id: Vec<u8>,
        name: String,
        description: String,
        main_unit_id: i64,
        essentiality_type_id: i64,
        quantity_type_id: i64,
        chemical_id: i64,
    ) -> Self {
        Self {
            nutrient_id,
            name,
            description,
            main_unit_id,
            essentiality_type_id,
            quantity_type_id,
            chemical_id,
        }
    }

    pub async fn from_nutrient(nutrient: Nutrient, pool: &Pool<Sqlite>) -> Result<Self, &'static str> {
        let nutrient_id = Id::<Nutrient>::new(InnerIdType::Uuid).to_bytes().to_vec();

        let name = nutrient.get_name();
        let description = nutrient.get_description();
        let nutrient_type_record = NutrientTypeRecord::from_nutrient_type(nutrient.get_nutrient_type());
        let main_unit_id = NutrientUnitRecord::from_nutrient_unit(nutrient.get_main_unit(), pool)
            .await
            .get_database_id(&pool)
            .await
            .unwrap();

        let essentiality_type_id = nutrient_type_record.get_essentiality_type_id();
        let quantity_type_id = nutrient_type_record.get_quantity_type_id();
        let chemical_id = nutrient_type_record.get_chemical_id_from_database(&pool).await.unwrap();

        Ok(Self {
            nutrient_id,
            name,
            description,
            essentiality_type_id,
            quantity_type_id,
            chemical_id,
            main_unit_id,
        })
    }

    pub async fn from_nutrient_entity(nutrient_entity: Entity<Nutrient>, pool: &Pool<Sqlite>) -> Result<Self, &'static str> {
        let nutrient_id = nutrient_entity.get_id().get_inner().to_bytes().to_vec();

        let nutrient = nutrient_entity.get_inner();
        let name = nutrient.get_name();
        let description = nutrient.get_description();
        let nutrient_type_record = NutrientTypeRecord::from_nutrient_type(nutrient.get_nutrient_type());
        let main_unit_id = NutrientUnitRecord::from_nutrient_unit(nutrient.get_main_unit(), pool)
            .await
            .get_database_id(&pool)
            .await
            .unwrap();

        let essentiality_type_id = nutrient_type_record.get_essentiality_type_id();
        let quantity_type_id = nutrient_type_record.get_quantity_type_id();
        let chemical_id = nutrient_type_record.get_chemical_id_from_database(&pool).await.unwrap();

        Ok(Self {
            nutrient_id,
            name,
            description,
            essentiality_type_id,
            quantity_type_id,
            chemical_id,
            main_unit_id,
        })
    }

    pub async fn to_nutrient(&self, pool: &Pool<Sqlite>) -> Nutrient {
        let name = self.name.clone();
        let description = self.description.clone();
        let nutrient_type_record =
            NutrientTypeRecord::load_from_database_from_nutrient_composite_id(
                self.essentiality_type_id,
                self.quantity_type_id,
                self.chemical_id,
                pool,
            )
            .await
            .unwrap();
        let nutrient_type = nutrient_type_record.to_nutrient_type();
        let main_unit_id = NutrientUnitRecord::load_from_database(self.main_unit_id, pool)
            .await
            .unwrap()
            .to_nutrient_unit(pool)
            .await;

        let mut nutrient = Nutrient::new(name, nutrient_type, main_unit_id);
        nutrient.set_description(description);

        return nutrient;
    }

    pub async fn to_nutrient_entity(&self, pool: &Pool<Sqlite>) -> Entity<Nutrient> {
        let name = self.name.clone();
        let description = self.description.clone();
        let nutrient_type_record =
            NutrientTypeRecord::load_from_database_from_nutrient_composite_id(
                self.essentiality_type_id,
                self.quantity_type_id,
                self.chemical_id,
                pool,
            )
            .await
            .unwrap();
        let nutrient_type = nutrient_type_record.to_nutrient_type();
        let main_unit_id = NutrientUnitRecord::load_from_database(self.main_unit_id, pool)
            .await
            .unwrap()
            .to_nutrient_unit(pool)
            .await;

        let mut nutrient = Nutrient::new(name, nutrient_type, main_unit_id);
        nutrient.set_description(description);

        // let id = Uuid::from_slice(&self.nutrient_id).unwrap();
        let id = Id::<Nutrient>::from_slice(InnerIdType::Uuid, &self.nutrient_id).unwrap();
        let nutrient_entity = Entity::<Nutrient>::new_with_id(id, nutrient);

        return nutrient_entity;
    }

    pub async fn load_from_database_using_name(name: String, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        Ok(sqlx::query_as!(
            NutrientRecord,
            r#"
                    SELECT
                        id as nutrient_id,
                        name,
                        description,
                        main_unit_id,
                        quantity_type_id,
                        essentiality_type_id,
                        chemical_id
                    FROM nutrients_nutrient_table
                    WHERE
                        name = ?
                "#,
            name
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn load_from_database_using_id(nutrient_id: Id<Nutrient>, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        let id = nutrient_id.get_inner().to_bytes().to_vec();

        Ok(sqlx::query_as!(
            NutrientRecord,
            r#"
                    SELECT
                        id as nutrient_id,
                        name,
                        description,
                        main_unit_id,
                        quantity_type_id,
                        essentiality_type_id,
                        chemical_id
                    FROM nutrients_nutrient_table
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
                INSERT INTO nutrients_nutrient_table (id, name, description, main_unit_id, quantity_type_id, essentiality_type_id, chemical_id)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT (name)
                DO UPDATE SET
                    description = excluded.description,
                    main_unit_id = excluded.main_unit_id,
                    quantity_type_id = excluded.quantity_type_id,
                    essentiality_type_id = excluded.essentiality_type_id,
                    chemical_id = excluded.chemical_id
                RETURNING id
            "#,
            self.nutrient_id,
            self.name,
            self.description,
            self.main_unit_id,
            self.quantity_type_id,
            self.essentiality_type_id,
            self.chemical_id,
        )
            .fetch_one(pool)
            .await?;
        Ok(row.id)
    }

    pub async fn delete_nutrient_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM nutrients_nutrient_table WHERE id = ?",
            self.nutrient_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct NutrientConversionRecord {
    pub(crate) nutrient_id: Vec<u8>,
    pub(crate) unit_id: i64,
    pub(crate) factor: f64,
}

impl NutrientConversionRecord {
    pub fn from_values(nutrient_id: Vec<u8>, unit_id: i64, factor: f64) -> Self {
        Self { nutrient_id, unit_id, factor }
    }

    pub async fn from_nutrient(nutrient: Nutrient, pool: &Pool<Sqlite>) -> Result<Vec<Self>, &'static str> {
        let nutrient_id = Id::<Nutrient>::new(InnerIdType::Uuid).to_bytes().to_vec();
        let mut conversion_vec = Vec::new();

        for (unit, factor) in nutrient.get_unit_conversions().iter() {
            let unit_id = NutrientUnitRecord::from_nutrient_unit(*unit, pool)
                .await
                .get_database_id(&pool)
                .await
                .expect("invalid nutrient");
            let factor = *factor;
            let conversion = Self {
                nutrient_id: nutrient_id.clone(),
                unit_id,
                factor,
            };
            conversion_vec.push(conversion);
        }

        Ok(conversion_vec)
    }

    pub async fn from_nutrient_entity(nutrient_entity: Entity<Nutrient>, pool: &Pool<Sqlite>) -> Result<Vec<Self>, &'static str> {
        let nutrient_id = nutrient_entity.get_id().get_inner().to_bytes().to_vec();
        let nutrient = nutrient_entity.get_inner();

        let mut conversion_vec = Vec::new();

        for (unit, factor) in nutrient.get_unit_conversions().iter() {
            println!("unit: {:#?}", unit);
            let unit_id = NutrientUnitRecord::from_nutrient_unit(*unit, pool)
                .await
                .get_database_id(&pool)
                .await
                .expect("invalid nutrient");
            let factor = *factor;
            let conversion = Self {
                nutrient_id: nutrient_id.clone(),
                unit_id,
                factor,
            };
            conversion_vec.push(conversion);
        }

        Ok(conversion_vec)
    }

    pub async fn to_btree_map_from_vec(items: Vec<Self>, pool: &Pool<Sqlite>) -> Result<BTreeMap<NutrientUnit, f64>, sqlx::Error> {
        let mut map: BTreeMap<NutrientUnit, f64> = BTreeMap::new();
        for conversion in items.iter() {
            let unit = NutrientUnitRecord::load_from_database(conversion.unit_id, pool)
                .await?
                .to_nutrient_unit(pool)
                .await;
            let factor = conversion.factor;
            map.insert(unit, factor);
        }

        return Ok(map);
    }

    pub fn sort_records(items: &mut Vec<Self>) {
        items.sort_by(|a, b| {
            a.nutrient_id
                .cmp(&b.nutrient_id)
                .then(a.unit_id.cmp(&b.unit_id))
                .then(a.factor.partial_cmp(&b.factor).unwrap())
        });
    }

    pub async fn load_from_database(nutrient_id: Id<Nutrient>, pool: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
        let id = nutrient_id.get_inner().to_bytes().to_vec();

        Ok(sqlx::query_as!(
            NutrientConversionRecord,
            r#"
                    SELECT
                        nutrient_id,
                        unit_id,
                        factor
                    FROM nutrients_unit_conversions
                    WHERE
                        nutrient_id = ?
                "#,
            id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_unit_conversions (nutrient_id, unit_id, factor)
                VALUES (?, ?, ?)
                ON CONFLICT (nutrient_id, unit_id)
                DO UPDATE SET
                    nutrient_id = excluded.nutrient_id,
                    unit_id = excluded.unit_id,
                    factor = excluded.factor
            "#,
            self.nutrient_id,
            self.unit_id,
            self.factor
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn save_vec_to_database(items: &Vec<Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for item in items {
            sqlx::query!(
                r#"
                    INSERT INTO nutrients_unit_conversions (nutrient_id, unit_id, factor)
                    VALUES (?, ?, ?)
                    ON CONFLICT (nutrient_id, unit_id)
                    DO UPDATE SET
                        unit_id = excluded.unit_id,
                        factor = excluded.factor
                "#,
                item.nutrient_id,
                item.unit_id,
                item.factor
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_conversion_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM nutrients_unit_conversions 
                WHERE
                    nutrient_id = ?
                    AND unit_id = ?
            "#,
            self.nutrient_id,
            self.unit_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_all_conversions_from_database(nutrient_id: Id<Nutrient>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let id = nutrient_id.get_inner().to_bytes().to_vec();

        sqlx::query!(
            r#"
                DELETE FROM nutrients_unit_conversions 
                WHERE
                    nutrient_id = ?
            "#,
            id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_conversion_vec_from_database(items: Vec<&Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for item in items {
            sqlx::query!(
                r#"
                DELETE FROM nutrients_unit_conversions 
                WHERE
                nutrient_id = ?
                    AND unit_id = ?
                    "#,
                item.nutrient_id,
                item.unit_id,
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;

        Ok(())
    }
}

pub struct NutrientLinkRecord {
    pub nutrient_id: Vec<u8>,
    pub parent_ids: Vec<Vec<u8>>,
    pub child_ids: Vec<Vec<u8>>,
}

impl NutrientLinkRecord {
    pub async fn from_nutrient(nutrient: Nutrient, pool: &Pool<Sqlite>) -> Result<Self, &'static str> {
        let nutrient_id = Id::<Nutrient>::new(InnerIdType::Uuid).to_bytes().to_vec();
        let mut child_ids: Vec<Vec<u8>> = Vec::new();
        for child in nutrient.get_children() {
            child_ids.push(
                NutrientRecord::from_nutrient(child.borrow().clone(), pool)
                    .await
                    .unwrap()
                    .nutrient_id,
            )
        }

        let mut parent_ids: Vec<Vec<u8>> = Vec::new();
        for parent_weak in nutrient.get_parents() {
            if let Some(parent) = parent_weak.upgrade() {
                parent_ids.push(
                    NutrientRecord::from_nutrient(parent.borrow().clone(), pool)
                        .await
                        .unwrap()
                        .nutrient_id,
                )
            } else {
                return Err("Weak parent");
            }
        }

        Ok(NutrientLinkRecord {
            nutrient_id,
            parent_ids,
            child_ids,
        })
    }

    pub async fn from_nutrient_entity(nutrient_entity: Entity<Nutrient>, pool: &Pool<Sqlite>) -> Result<Self, &'static str> {
        let nutrient_id = nutrient_entity.get_id().get_inner().to_bytes().to_vec();
        let nutrient = nutrient_entity.get_inner();

        let mut child_ids: Vec<Vec<u8>> = Vec::new();
        for child in nutrient.get_children() {
            child_ids.push(
                NutrientRecord::from_nutrient(child.borrow().clone(), pool)
                    .await
                    .unwrap()
                    .nutrient_id,
            )
        }

        let mut parent_ids: Vec<Vec<u8>> = Vec::new();
        for parent_weak in nutrient.get_parents() {
            if let Some(parent) = parent_weak.upgrade() {
                parent_ids.push(
                    NutrientRecord::from_nutrient(parent.borrow().clone(), pool)
                        .await
                        .unwrap()
                        .nutrient_id,
                )
            } else {
                return Err("Weak parent");
            }
        }

        Ok(NutrientLinkRecord {
            nutrient_id,
            parent_ids,
            child_ids,
        })
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for child_id in &self.child_ids {
            sqlx::query!(
                r#"
                    INSERT INTO nutrients_nutrient_relationships (parent_id, child_id)
                    VALUES (?, ?)
                    ON CONFLICT (parent_id, child_id)
                    DO NOTHING
                "#,
                self.nutrient_id,
                child_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        for parent_id in &self.parent_ids {
            sqlx::query!(
                r#"
                    INSERT INTO nutrients_nutrient_relationships (parent_id, child_id)
                    VALUES (?, ?)
                    ON CONFLICT (parent_id, child_id)
                    DO NOTHING
                "#,
                parent_id,
                self.nutrient_id,
            )
            .execute(&mut *tx)
            .await?;
        }
        let _ = tx.commit().await;

        Ok(())
    }

    pub async fn load_from_sqlite(nutrient_id: Uuid, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        let parent_rows = sqlx::query!(
            r#"
                SELECT
                    parent_id
                FROM nutrients_nutrient_relationships
                WHERE
                    child_id = ?
            "#,
            nutrient_id
        )
        .fetch_all(pool)
        .await?;
        let parent_ids: Vec<Vec<u8>> = parent_rows
            .iter()
            .map(|row| row.parent_id.clone())
            .collect();

        let child_rows = sqlx::query!(
            r#"
                SELECT
                    child_id
                FROM nutrients_nutrient_relationships
                WHERE
                    parent_id = ?
            "#,
            nutrient_id
        )
        .fetch_all(pool)
        .await?;
        let child_ids: Vec<Vec<u8>> = child_rows.iter().map(|row| row.child_id.clone()).collect();

        let nutrient_link_record = Self {
            nutrient_id: nutrient_id.as_bytes().to_vec(),
            parent_ids,
            child_ids,
        };
        Ok(nutrient_link_record)
    }

    pub async fn delete_nutrient_link(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query!(
            r#"
                DELETE FROM nutrients_nutrient_relationships
                WHERE
                    parent_id = ?
            "#,
            self.nutrient_id,
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
                DELETE FROM nutrients_nutrient_relationships
                WHERE
                    child_id = ?
            "#,
            self.nutrient_id,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}

pub struct NutrientLinkRecordUuid {
    pub nutrient_id: Uuid,
    pub parent_id_vec: Vec<Uuid>,
    pub child_id_vec: Vec<Uuid>,
}
